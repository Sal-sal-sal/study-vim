local root = assert(vim.env.STUDY_TEST_ROOT)
vim.opt.runtimepath:prepend(vim.fn.getcwd())
vim.cmd("filetype plugin indent on")
require("study").setup({ root = root, binary = vim.env.STUDY_TEST_BINARY })
vim.cmd("Study Linear algebra")
local plan = vim.fs.joinpath(root, "Linear algebra", "Linear algebra.study")
assert(vim.uv.fs_realpath(vim.api.nvim_buf_get_name(0)) == vim.uv.fs_realpath(plan))
assert(vim.bo.filetype == "markdown")
assert(vim.fn.getcwd():match("Linear algebra$"))
local lines = { "# Linear algebra", "", "## Ссылки", "", "[Course](https://example.org)", "", "My notes" }
vim.api.nvim_buf_set_lines(0, 0, -1, false, lines)
local url
vim.ui.open = function(value)
  url = value
end
vim.api.nvim_win_set_cursor(0, { 5, 3 })
vim.cmd.StudyOpen()
assert(url == "https://example.org")
vim.api.nvim_win_set_cursor(0, { 1, 2 })
assert(require("study.commands").open() == false)
vim.cmd.write()
vim.cmd("Study Linear algebra")
assert(vim.api.nvim_buf_get_lines(0, 6, 7, false)[1] == "My notes")
print("study.nvim command tests passed")
