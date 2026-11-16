# Yapfast — testing

Эта ветка моего форка нужна для выпуска собственных тестовых релизов Yapfast.
Здесь объединяются изменения из моих PR, чтобы проверять их совместную работу
и выявлять, не ломают ли они друг друга.

Релизы собираются для Windows и Linux. Для Linux выпускаются AppImage и пакет
для Arch Linux. Сборки можно запускать через GitHub Actions; они предназначены
для тестирования, а не для стабильного использования.

Список влитых веток ведётся в [CHANGELOG.md](CHANGELOG.md).
Подробнее об обновлении веток и выпуске сборок — в [инструкции](docs/fork-testing.md).

## Обновление из upstream/main

```powershell
git fetch upstream
git switch testing
git merge upstream/main
git push
```
