# study.nvim

Учебные папки и планы в Neovim с логикой на Rust.
Файл `.study` остаётся обычным Markdown: заголовки, списки, код и ссылки работают привычным образом.
Плагин добавляет создание тем, переходы по папкам и свои цвета.

## Установка

Нужны Rust 1.88 или новее, Cargo и Neovim 0.10 или новее.
Поддерживаются Linux, macOS и Windows.

Для lazy.nvim:

```lua
return {
  {
    "Sal-sal-sal/study-vim",
    name = "study.nvim",
    lazy = false,
    build = "cargo build --release --locked",
    opts = {},
  },
}
```

Для команды `study` в терминале установи исполняемый файл:

```sh
git clone https://github.com/Sal-sal-sal/study-vim.git
cd study-vim
cargo install --path . --locked
study ML
```

Эти команды подходят и для PowerShell.
Папка Cargo `bin` должна находиться в `PATH`, как при стандартной установке Rust.
Для запуска из терминала `nvim` тоже должен находиться в `PATH`.
Если сборка плагина ещё не выполнена, запусти `:StudyBuild` внутри Neovim.

## Первые шаги

1. Выполни `study ML` в терминале или `:Study ML` в Neovim.
   Появятся `~/study/ML/` и `~/study/ML/ML.study`, а редактор откроет план внутри папки темы.
2. Запиши материалы под заголовком `## Ссылки`.
   Например: `[Курс](https://developers.google.com/machine-learning/crash-course)`.
3. Выполни `:StudyFolder Linear algebra`.
   Команда создаст подпапку с её собственным планом и добавит ссылку под `## Папки`.
4. Поставь курсор на ссылку и нажми Enter или выполни `:StudyOpen`.
   Веб-ссылка откроется в системном браузере, файл в редакторе, а папка во встроенном списке файлов.
5. В списке файлов нажимай Enter для открытия, `-` для перехода вверх и `q` для возврата к плану.
6. Сохраняй изменения обычной командой `:write`.
   Повторное открытие темы сохранит существующий план.

Темы с пробелами в терминале заключай в кавычки: `study "Linear algebra"`.
В Neovim кавычки не нужны: `:Study Linear algebra`.
Enter вне ссылки сохраняет обычное поведение перехода на следующую строку.

## Формат плана

```markdown
# ML

## Ссылки

- [Курс](https://developers.google.com/machine-learning/crash-course)
- [Заметки](./notes.md#intro)

## Папки

- [Алгебра](./linear-algebra/)
- [Папка с пробелом](<./Linear algebra/>)
```

Пути считаются относительно файла плана, а не текущей папки терминала.
Поддерживаются обычные и ссылочные Markdown-ссылки, `<https://example.org>`, URL-кодирование путей и переходы к заголовкам.
Для заголовка `## Linear algebra` якорь имеет вид `#linear-algebra`, а для второго такого же заголовка `#linear-algebra-1`.
Ссылки внутри блоков кода не открываются.
Папки для ссылок можно создать вручную или через `:StudyFolder`.
Готовый пример лежит в [examples/ML.study](examples/ML.study).

## Настройка

По умолчанию используется домашняя папка текущего пользователя плюс `study`.
Жёсткой привязки к имени пользователя или `/Users` нет.
Приоритет выбора папки: `--root` или Lua `root`, затем `STUDY_ROOT`, затем `~/study`.

```sh
study ML --root ./learning
study ML --editor /path/to/nvim
```

`STUDY_NVIM` задаёт исполняемый файл Neovim для команды в терминале.
Значение представляет один путь к программе, без дополнительных аргументов оболочки.

```lua
require("study").setup({
  root = "~/courses",
  -- binary = "/path/to/study", -- На Windows можно указать study.exe.
  timeout = 5000,
  mappings = { open = "<CR>" }, -- false отключает эту привязку.
  colors = {
    text = "#FFFFFF",
    link = "#61AFEF",
    heading = "#FFFFFF",
  },
})
```

Белый текст, синие ссылки и жирные заголовки применяются только к окнам `.study` и встроенного списка папок.
Цвет фона берётся из текущей темы оформления.
Для обычных `.md` и остальных файлов сохраняется их подсветка.
Подсветка работает через стандартный Markdown-синтаксис или уже установленный Treesitter.
Дополнительный файловый плагин не требуется.

## Разработка и проверка

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
python scripts/check_nvim.py
stylua --check lua plugin tests/nvim
```

В CI эти проверки выполняются на Linux, macOS и Windows с минимальной и текущей стабильной версиями Neovim.
Rust отвечает за пути, создание файлов и разбор Markdown.
Lua отвечает за команды Neovim, окна, клавиши и подсветку.
Слои общаются JSON-запросами без Bash и загрузки платформенной библиотеки в Neovim.
Подробнее о реализации: [docs/implementation.md](docs/implementation.md).
