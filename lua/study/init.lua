local M = {}

function M.setup(options)
  if vim.fn.has("nvim-0.10") ~= 1 then
    vim.notify("study.nvim requires Neovim 0.10 or newer", vim.log.levels.ERROR)
    return
  end
  require("study.config").setup(options)
  vim.filetype.add({ extension = { study = "markdown" } })
  require("study.commands").setup()
  require("study.buffers").setup()
  require("study.style").setup()
end

return M
