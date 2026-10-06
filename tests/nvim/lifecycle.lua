local api = vim.api
local root = assert(vim.env.STUDY_TEST_ROOT)
vim.opt.runtimepath:prepend(vim.fn.getcwd())
vim.cmd("filetype plugin indent on")
vim.cmd("runtime plugin/study.lua")
assert(vim.g.loaded_study)
vim.cmd("runtime plugin/study.lua")
require("study").setup({ root = root, binary = vim.env.STUDY_TEST_BINARY })
vim.cmd("Study ML")
assert(vim.fn.maparg("<CR>", "n", false, true).buffer == 1)
require("study").setup({
  root = root,
  binary = vim.env.STUDY_TEST_BINARY,
  mappings = { open = false },
})
assert(vim.fn.maparg("<CR>", "n", false, true).buffer == nil)
local original = api.nvim_create_namespace("study.test.other")
local namespace = api.nvim_create_namespace("study.style")
api.nvim_win_set_hl_ns(0, original)
vim.cmd.enew()
assert(api.nvim_get_hl_ns({ winid = 0 }) == original)
local notices = {}
vim.notify = function(message)
  table.insert(notices, message)
end
require("study").setup({ binary = vim.fs.joinpath(root, "missing-executable") })
vim.cmd("Study Missing")
assert(#notices == 1)
assert(api.nvim_get_hl_ns({ winid = 0 }) ~= namespace)
print("study.nvim loading and error lifecycle tests passed")
