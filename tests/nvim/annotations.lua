local api = vim.api
vim.opt.runtimepath:prepend(vim.fn.getcwd())
vim.cmd("filetype plugin indent on")
require("study").setup({ binary = vim.env.STUDY_TEST_BINARY })
local namespace = api.nvim_create_namespace("study.annotations")
vim.cmd.edit(vim.fn.fnameescape(vim.fs.joinpath(vim.env.STUDY_TEST_ROOT, "annotations.study")))
local buffer = api.nvim_get_current_buf()
api.nvim_buf_set_lines(buffer, 0, -1, false, {
  "# ML",
  ">! Заметка",
  "",
  "```text",
  ">! Пример в коде",
  "```",
})
local function marks()
  return api.nvim_buf_get_extmarks(buffer, namespace, 0, -1, { details = true })
end
local tick = api.nvim_buf_get_changedtick(buffer)
assert(vim.wait(3000, function()
  return #marks() == 2
end))
assert(marks()[1][2] == 1 and marks()[2][2] == 1)
assert(api.nvim_buf_get_changedtick(buffer) == tick)
assert(api.nvim_buf_get_lines(buffer, 1, 2, false)[1] == ">! Заметка")
api.nvim_buf_set_lines(buffer, 1, 2, false, { "Обычная строка" })
assert(vim.wait(3000, function()
  return #marks() == 0
end))
api.nvim_buf_set_lines(buffer, 1, 2, false, { ">! Новая заметка" })
assert(vim.wait(3000, function()
  return #marks() == 2
end))
vim.cmd.file(vim.fn.fnameescape(vim.fs.joinpath(vim.env.STUDY_TEST_ROOT, "annotations.md")))
assert(#marks() == 0)
vim.cmd.file(vim.fn.fnameescape(vim.fs.joinpath(vim.env.STUDY_TEST_ROOT, "annotations.study")))
assert(vim.wait(3000, function()
  return #marks() == 2
end))
require("study").setup({ binary = vim.env.STUDY_TEST_BINARY })
assert(vim.wait(3000, function()
  return #marks() == 2
end))
api.nvim_buf_set_lines(buffer, 1, 2, false, { ">! Быстрое изменение" })
api.nvim_buf_set_lines(buffer, 1, 2, false, { "Без маркера" })
assert(vim.wait(3000, function()
  return #marks() == 0
end))
print("study.nvim note rendering and edit lifecycle tests passed")

api.nvim_buf_set_lines(buffer, 0, -1, false, {
  "- [ ] Pending",
  "- [x] Готово",
  "~~Повторил~~",
  "`~~Код~~`",
  "```",
  "- [x] Пример",
  "```",
})
assert(vim.wait(3000, function()
  local current = marks()
  return #current == 2 and current[1][2] == 1 and current[2][2] == 2
end))
vim.bo[buffer].undolevels = vim.bo[buffer].undolevels
api.nvim_buf_set_lines(buffer, 1, 3, false, { "- [ ] Готово", "Повторил" })
assert(vim.wait(3000, function()
  return #marks() == 0
end))
vim.cmd("normal! u")
assert(vim.wait(3000, function()
  return #marks() == 2
end))
assert(api.nvim_buf_get_lines(buffer, 1, 2, false)[1] == "- [x] Готово")
local styled_tick = api.nvim_buf_get_changedtick(buffer)
vim.cmd("doautocmd ColorScheme")
assert(api.nvim_buf_get_changedtick(buffer) == styled_tick)
assert(#marks() == 2)
print("study.nvim completed tasks, strikes, edits and undo tests passed")
