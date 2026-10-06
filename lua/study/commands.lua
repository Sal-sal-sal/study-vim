local M = {}
local api = vim.api
local backend = require("study.backend")

local function report(error)
  vim.notify("study.nvim: " .. tostring(error), vim.log.levels.ERROR)
end

function M.create(topic)
  local result, error = backend.request({
    action = "create",
    topic = topic,
    root = require("study.config").options.root,
  })
  if error then
    report(error)
    return
  end
  vim.cmd.edit(vim.fn.fnameescape(result.file))
  vim.cmd.lcd(vim.fn.fnameescape(result.directory))
end

function M.open(explicit)
  local buffer = api.nvim_get_current_buf()
  local document = api.nvim_buf_get_name(buffer)
  if not document:match("%.study$") then
    if explicit then
      report("Open a .study file first")
    end
    return false
  end
  local cursor = api.nvim_win_get_cursor(0)
  local target, error = backend.request({
    action = "open",
    document = document,
    text = table.concat(api.nvim_buf_get_lines(buffer, 0, -1, false), "\n"),
    line = cursor[1],
    column = cursor[2],
  })
  if error then
    report(error)
    return true
  end
  if not target or target == vim.NIL then
    if explicit then
      vim.notify("study.nvim: there is no link under the cursor")
    end
    return false
  end
  local open_error = require("study.navigation").open(target)
  if open_error then
    report(open_error)
  end
  return true
end

function M.folder(topic)
  local document = api.nvim_buf_get_name(0)
  if not document:match("%.study$") or not vim.bo.modifiable then
    report("Open an editable .study file first")
    return
  end
  local result, error = backend.request({ action = "folder", document = document, topic = topic })
  if error then
    report(error)
    return
  end
  local lines = api.nvim_buf_get_lines(0, 0, -1, false)
  if vim.tbl_contains(lines, result.link) then
    return
  end
  local start, ending
  for index, line in ipairs(lines) do
    if line == "## Папки" then
      start = index
    elseif start and line:match("^##? ") then
      ending = index - 1
      break
    end
  end
  if not start then
    api.nvim_buf_set_lines(0, -1, -1, false, { "", "## Папки", "", result.link })
  else
    local insert = ending or #lines
    api.nvim_buf_set_lines(0, insert, insert, false, { result.link })
  end
end

function M.setup()
  api.nvim_create_user_command("StudyFolder", function(command)
    M.folder(command.args)
  end, { nargs = "+", force = true, desc = "Create a subtopic and add its folder link" })
  api.nvim_create_user_command("Study", function(command)
    M.create(command.args)
  end, { nargs = "+", force = true, desc = "Create or open a study topic" })
  api.nvim_create_user_command("StudyOpen", function()
    M.open(true)
  end, { force = true, desc = "Follow the study link under the cursor" })
  api.nvim_create_user_command(
    "StudyBuild",
    backend.build,
    { force = true, desc = "Build the Rust study backend" }
  )
end

return M
