local M = {}
local api = vim.api

local function heading(fragment)
  local result, error = require("study.backend").request({
    action = "heading",
    text = table.concat(api.nvim_buf_get_lines(0, 0, -1, false), "\n"),
    fragment = fragment,
  })
  if error then
    return error
  end
  api.nvim_win_set_cursor(0, { result.line, 0 })
end

function M.open(target)
  if target.kind == "url" then
    local _, error = vim.ui.open(target.url)
    return error
  elseif target.kind == "file" then
    vim.cmd.edit(vim.fn.fnameescape(target.path))
    if target.fragment and target.fragment ~= vim.NIL then
      return heading(target.fragment)
    end
  elseif target.kind == "directory" then
    vim.cmd.edit(vim.fn.fnameescape(target.path))
  elseif target.kind == "anchor" then
    return heading(target.fragment)
  end
end

return M
