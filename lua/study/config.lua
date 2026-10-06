local M = {}
local defaults = {
  root = nil,
  binary = nil,
  timeout = 5000,
  mappings = { open = "<CR>" },
  colors = { text = "#FFFFFF", link = "#61AFEF", heading = "#FFFFFF" },
}

M.options = vim.deepcopy(defaults)

function M.setup(options)
  M.options = vim.tbl_deep_extend("force", vim.deepcopy(defaults), options or {})
end

return M
