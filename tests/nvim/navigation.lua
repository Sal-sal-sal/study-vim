local api = vim.api
local root = assert(vim.env.STUDY_TEST_ROOT)
vim.opt.runtimepath:prepend(vim.fn.getcwd())
vim.cmd("filetype plugin indent on")
require("study").setup({ root = root, binary = vim.env.STUDY_TEST_BINARY })
vim.cmd("Study ML")
local plan_buffer = api.nvim_get_current_buf()
local topic = vim.fs.dirname(api.nvim_buf_get_name(0))
local folder = vim.fs.joinpath(topic, "Linear algebra")
vim.fn.mkdir(folder, "p")
vim.fn.writefile({ "# Notes", "", "## Intro", "", "Text" }, vim.fs.joinpath(folder, "notes.md"))
api.nvim_buf_set_lines(0, 0, -1, false, {
  "# ML",
  "",
  "## Папки",
  "",
  "[Algebra](<./Linear algebra/>)",
  "",
  "[Section](#папки)",
})
vim.o.hidden = true
api.nvim_win_set_cursor(0, { 7, 3 })
vim.cmd.StudyOpen()
assert(api.nvim_win_get_cursor(0)[1] == 3)
api.nvim_win_set_cursor(0, { 5, 3 })
vim.cmd.StudyOpen()
assert(vim.bo.filetype == "study-directory")
assert(vim.b.study_directory:match("Linear algebra/?$"))
api.nvim_win_set_cursor(0, { 5, 0 })
local mapping = vim.fn.maparg("<CR>", "n", false, true)
mapping.callback()
assert(vim.fs.basename(api.nvim_buf_get_name(0)) == "notes.md")
api.nvim_set_current_buf(plan_buffer)
api.nvim_buf_set_lines(0, 4, 5, false, { "[Notes](<./Linear algebra/notes.md#intro>)" })
api.nvim_win_set_cursor(0, { 5, 3 })
vim.cmd.StudyOpen()
assert(api.nvim_win_get_cursor(0)[1] == 3)
api.nvim_set_current_buf(plan_buffer)
api.nvim_buf_set_lines(0, 4, 5, false, { "[Algebra](<./Linear algebra/>)" })
api.nvim_win_set_cursor(0, { 5, 3 })
vim.cmd.StudyOpen()
vim.fn.maparg("-", "n", false, true).callback()
assert(vim.b.study_directory:match("ML/?$"))
vim.fn.maparg("q", "n", false, true).callback()
assert(api.nvim_get_current_buf() == plan_buffer)
assert(api.nvim_buf_get_lines(0, 2, 3, false)[1] == "## Папки")
print("study.nvim folder and heading navigation tests passed")
