local api = vim.api
vim.opt.runtimepath:prepend(vim.fn.getcwd())
vim.cmd("filetype plugin indent on")
vim.cmd("syntax enable")
require("study").setup()
local namespace = api.nvim_create_namespace("study.style")
local original = api.nvim_create_namespace("study.test.original")
api.nvim_set_hl(0, "Normal", { fg = "#AABBCC", bg = "#101820" })
vim.wo.spell = true
api.nvim_win_set_hl_ns(0, original)
vim.cmd.edit(vim.fn.fnameescape(vim.fs.joinpath(vim.env.STUDY_TEST_ROOT, "visual.study")))
api.nvim_buf_set_lines(
  0,
  0,
  -1,
  false,
  { "# ML", "", "Plain text", "", "[Course](https://example.org)" }
)
assert(vim.bo.filetype == "markdown")
assert(api.nvim_get_hl_ns({ winid = 0 }) == namespace)
require("study").setup()
local normal = api.nvim_get_hl(namespace, { name = "Normal" })
assert(normal.fg == 0xFFFFFF and normal.bg == 0x101820)
assert(api.nvim_get_hl(namespace, { name = "@markup.link.url.markdown_inline" }).fg == 0x61AFEF)
assert(api.nvim_get_hl(namespace, { name = "markdownLinkText" }).fg == 0x61AFEF)
assert(not vim.wo.spell)
assert(api.nvim_get_hl(namespace, { name = "@markup.heading.1.markdown" }).bold)
vim.cmd.vsplit()
assert(api.nvim_get_hl_ns({ winid = 0 }) == namespace)
vim.cmd.enew()
assert(api.nvim_get_hl_ns({ winid = 0 }) ~= namespace)
vim.cmd.close()
vim.cmd("doautocmd ColorScheme")
assert(api.nvim_get_hl(namespace, { name = "Normal" }).fg == 0xFFFFFF)
local plain = vim.fs.joinpath(vim.env.STUDY_TEST_ROOT, "visual.md")
vim.cmd.file(vim.fn.fnameescape(plain))
assert(api.nvim_get_hl_ns({ winid = 0 }) == original)
assert(vim.wo.spell)
assert(vim.fn.maparg("<CR>", "n", false, true).buffer == nil)
assert(api.nvim_get_hl(0, { name = "Normal" }).fg == 0xAABBCC)
require("study").setup({ colors = { text = "#EEEEEE", link = "#123ABC" } })
assert(api.nvim_get_hl(namespace, { name = "Normal" }).fg == 0xEEEEEE)
assert(api.nvim_get_hl(namespace, { name = "markdownUrl" }).fg == 0x123ABC)
print("study.nvim style lifecycle tests passed")
