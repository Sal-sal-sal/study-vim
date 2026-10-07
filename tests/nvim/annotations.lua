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
