//! Builds the Go Live codec shims, the GIF thumbnail encoder, the video file
//! reader and the cameras against the libvpx, FFmpeg, dav1d, libyuv and
//! WebRTC headers in the same libwebrtc archive webrtc-sys links, so they
//! match its code, and the H264 encoder against x264 (vendor/x264-src).

use std::{env, path::PathBuf, process::Command};

fn main() {
    for file in [
        "vp8.c",
        "h264.c",
        "clip.c",
        "decode.c",
        "av1.c",
        "video.c",
        "picture.c",
        "camera.cc",
        "hwenc.c",
        "hevc.c",
        "video_hevc.c",
        "video_hevc.h",
    ] {
        println!("cargo:rerun-if-changed=native/{file}");
    }
    let include = webrtc_sys_build::webrtc_dir().join("include");
    let third_party = include.join("third_party");
    let mut picture = cc::Build::new();
    picture
        .file("native/picture.c")
        .include(third_party.join("libyuv/include"))
        .warnings(true);
    use_clang_cl(&mut picture);
    picture.compile("yapfast_picture");
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    // Linux captures through V4L2, and Windows through DirectShow, both in
    // the same module of the libwebrtc archive.
    if matches!(target_os.as_str(), "linux" | "windows") {
        let mut camera = cc::Build::new();
        camera
            .cpp(true)
            .file("native/camera.cc")
            .include(&include)
            .include(third_party.join("abseil-cpp"))
            .include(third_party.join("libyuv/include"))
            .std("c++20")
            .flag("-Wno-deprecated-declarations")
            // libwebrtc's headers mark some pointers and not others.
            .flag("-Wno-nullability-completeness")
            .warnings(true);
        use_clang_cl(&mut camera);
        // The toolchain sets its own hardening mode.
        for (key, value) in webrtc_sys_build::webrtc_defines() {
            if key != "_LIBCPP_HARDENING_MODE" {
                camera.define(&key, value.as_deref());
            }
        }
        camera.compile("yapfast_camera");
        // DirectShow's device list reads each camera's name and path as a
        // VARIANT; webrtc-sys links the rest.
        if target_os == "windows" {
            println!("cargo:rustc-link-lib=dylib=oleaut32");
        }
    }
    println!("cargo::rustc-check-cfg=cfg(hwenc)");
    println!("cargo::rustc-check-cfg=cfg(hevcdec)");
    if std::env::var_os("CARGO_FEATURE_DISCORD").is_none() {
        return;
    }
    // FFmpeg's hardware encoders and HEVC decoder, where ffmpeg-src builds
    // them: hwenc.c, hevc.c and video_hevc.c see only their headers, with
    // FFmpeg's names prefixed as in the library.
    if std::env::var("DEP_FFMPEG_ENCODERS").as_deref() == Ok("1") {
        let mut ffmpeg = cc::Build::new();
        use_clang_cl(&mut ffmpeg);
        ffmpeg
            .file("native/hwenc.c")
            .file("native/hevc.c")
            .file("native/video_hevc.c")
            .warnings(true);
        let folders = std::env::var_os("DEP_FFMPEG_INCLUDE").expect("ffmpeg-src's headers");
        ffmpeg.includes(std::env::split_paths(&folders));
        let rename = std::env::var("DEP_FFMPEG_RENAME").expect("ffmpeg-src's renames");
        // clang-cl, for Windows, spells the flags as MSVC does.
        if ffmpeg.get_compiler().is_like_msvc() {
            ffmpeg.flag("/std:c17").flag(format!("/FI{rename}"));
        } else {
            ffmpeg.flag("-std=c17").flag("-include").flag(&rename);
        }
        ffmpeg.compile("yapfast_hwenc");
        println!("cargo:rustc-cfg=hwenc");
        println!("cargo:rustc-cfg=hevcdec");
    }
    let os = match target_os.as_str() {
        "windows" => "win",
        "macos" => "mac",
        _ => "linux",
    };
    let arch = match std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("aarch64") => "arm64",
        _ => "x64",
    };
    let mut codecs = cc::Build::new();
    // LiveKit prefixes Chromium FFmpeg's Linux symbols with LK_ (since
    // webrtc-sys 0.3.50). Rename declarations and calls in these shims to
    // match, without touching the separately prefixed vendor/ffmpeg-src.
    if target_os == "linux" {
        for symbol in [
            "av_find_best_stream",
            "av_frame_alloc",
            "av_frame_free",
            "av_frame_unref",
            "av_free",
            "av_freep",
            "av_get_packed_sample_fmt",
            "av_log_set_level",
            "av_malloc",
            "av_packet_alloc",
            "av_packet_free",
            "av_packet_get_side_data",
            "av_packet_move_ref",
            "av_packet_unref",
            "av_pix_fmt_desc_get",
            "av_read_frame",
            "av_rescale_q",
            "av_sample_fmt_is_planar",
            "avcodec_alloc_context3",
            "avcodec_find_decoder",
            "avcodec_flush_buffers",
            "avcodec_free_context",
            "avcodec_open2",
            "avcodec_parameters_to_context",
            "avcodec_receive_frame",
            "avcodec_send_packet",
            "avformat_alloc_context",
            "avformat_close_input",
            "avformat_find_stream_info",
            "avformat_open_input",
            "avformat_seek_file",
            "avio_alloc_context",
            "avio_context_free",
        ] {
            codecs.define(symbol, Some(format!("LK_{symbol}").as_str()));
        }
    }
    if std::env::var("DEP_FFMPEG_ENCODERS").as_deref() == Ok("1") {
        codecs.define("YAPFAST_FILE_HEVC", None);
    }
    codecs
        .file("native/vp8.c")
        .file("native/h264.c")
        .file("native/clip.c")
        .file("native/decode.c")
        .file("native/av1.c")
        .file("native/video.c")
        .include(third_party.join("libvpx/source/libvpx"))
        // x264-src's build script hands over its headers' folder.
        .include(std::env::var_os("DEP_X264_INCLUDE").expect("x264-src's headers"))
        .include(third_party.join("ffmpeg"))
        .include(third_party.join("dav1d/libdav1d/include"))
        .include(third_party.join("libyuv/include"))
        // FFmpeg's configuration for the Chrome build libwebrtc uses.
        .include(third_party.join(format!("ffmpeg/chromium/config/Chrome/{os}/{arch}")))
        .warnings(true);
    use_clang_cl(&mut codecs);
    codecs.compile("yapfast_codecs");
}

// camera.cc uses Clang-only warning flags, which cl.exe rejects. Switch only
// cc's own MSVC default to clang-cl; a person's compiler choice wins.
fn use_clang_cl(build: &mut cc::Build) {
    if env::var("CARGO_CFG_TARGET_ENV").as_deref() != Ok("msvc") || has_explicit_compiler() {
        return;
    }
    if build.get_compiler().is_like_clang() {
        return;
    }
    let clang_cl = find_clang_cl().unwrap_or_else(|| {
        panic!(
            "this MSVC build needs clang-cl; install LLVM for Windows and add its bin directory to PATH, or install Visual Studio's C++ Clang tools for Windows component"
        )
    });
    build.compiler(clang_cl);
}

fn has_explicit_compiler() -> bool {
    if env::var_os("CC").is_some() || env::var_os("CXX").is_some() {
        return true;
    }
    let Ok(target) = env::var("TARGET") else {
        return false;
    };
    let underscored_target = target.replace('-', "_");
    ["CC", "CXX"].iter().any(|variable| {
        env::var_os(format!("{variable}_{target}")).is_some()
            || env::var_os(format!("{variable}_{underscored_target}")).is_some()
    })
}

fn find_clang_cl() -> Option<PathBuf> {
    env::var_os("PATH")
        .and_then(|path| {
            env::split_paths(&path)
                .map(|folder| folder.join("clang-cl.exe"))
                .find(|path| path.is_file())
        })
        .or_else(|| {
            env::var_os("VCINSTALLDIR")
                .map(|vc| PathBuf::from(vc).join("Tools/Llvm/x64/bin/clang-cl.exe"))
                .filter(|path| path.is_file())
        })
        .or_else(|| {
            let vswhere = PathBuf::from(env::var_os("ProgramFiles(x86)")?)
                .join("Microsoft Visual Studio/Installer/vswhere.exe");
            let output = Command::new(vswhere)
                .args([
                    "-latest",
                    "-products",
                    "*",
                    "-find",
                    r"**\Llvm\**\clang-cl.exe",
                ])
                .output()
                .ok()?;
            if !output.status.success() {
                return None;
            }
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(|line| PathBuf::from(line.trim()))
                .find(|path| path.is_file())
        })
        .or_else(|| {
            env::var_os("ProgramFiles")
                .map(|folder| PathBuf::from(folder).join("LLVM/bin/clang-cl.exe"))
                .filter(|path| path.is_file())
        })
}
