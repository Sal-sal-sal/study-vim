local M = {}
local api = vim.api
local highlights = api.nvim_create_namespace("study.directory")

local function report(error)
  vim.notify("study.nvim: " .. tostring(error), vim.log.levels.ERROR)
end

function M.open(path, origin)
  path = vim.fs.normalize(vim.fn.fnamemodify(path, ":p"))
  local directory, error = require("study.backend").request({ action = "browse", path = path })
  if error then
    return error
  end
  origin = origin or api.nvim_get_current_buf()
  local items = {}
  if directory.parent and directory.parent ~= vim.NIL then
    table.insert(items, { name = "..", path = directory.parent, directory = true })
  end
  vim.list_extend(items, directory.entries)
  local lines = { "# " .. (vim.fs.basename(path) or path), "Enter: open | -: parent | q: back", "" }
  for _, item in ipairs(items) do
    table.insert(lines, item.name .. (item.directory and "/" or ""))
  end
  if #items == 0 then
    table.insert(lines, "(empty directory)")
  end
  local buffer = api.nvim_create_buf(false, true)
  api.nvim_set_current_buf(buffer)
  vim.bo[buffer].buftype = "nofile"
  vim.bo[buffer].bufhidden = "wipe"
  vim.bo[buffer].swapfile = false
  api.nvim_buf_set_lines(buffer, 0, -1, false, lines)
  vim.bo[buffer].modifiable = false
  vim.bo[buffer].filetype = "study-directory"
  vim.b[buffer].study_directory = path
  api.nvim_set_hl(0, "StudyDirectoryTitle", { link = "Title", default = true })
  api.nvim_set_hl(0, "StudyDirectoryLink", { link = "Directory", default = true })
  api.nvim_buf_set_extmark(
    buffer,
    highlights,
    0,
    0,
    { end_row = 0, end_col = #lines[1], hl_group = "StudyDirectoryTitle" }
  )
  for index, item in ipairs(items) do
    if item.directory then
      api.nvim_buf_set_extmark(buffer, highlights, index + 2, 0, {
        end_row = index + 2,
        end_col = #lines[index + 3],
        hl_group = "StudyDirectoryLink",
      })
    end
  end
  vim.wo.wrap = false
  vim.wo.cursorline = true
  local function visit(target)
    if target.directory then
      local browse_error = M.open(target.path, origin)
      if browse_error then
        report(browse_error)
      end
    else
      vim.cmd.edit(vim.fn.fnameescape(target.path))
    end
  end
  vim.keymap.set("n", "<CR>", function()
    local item = items[api.nvim_win_get_cursor(0)[1] - 3]
    if item then
      visit(item)
    end
  end, { buffer = buffer, desc = "Open study entry" })
  vim.keymap.set("n", "-", function()
    if directory.parent and directory.parent ~= vim.NIL then
      visit({ path = directory.parent, directory = true })
    end
  end, { buffer = buffer, desc = "Parent study directory" })
  vim.keymap.set("n", "q", function()
    if api.nvim_buf_is_valid(origin) then
      api.nvim_set_current_buf(origin)
    else
      vim.cmd.enew()
    end
  end, { buffer = buffer, desc = "Return to study plan" })
  api.nvim_win_set_cursor(0, { 4, 0 })
end

return M
