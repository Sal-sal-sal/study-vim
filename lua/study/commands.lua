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
  if target.kind == "url" then
    local _, open_error = vim.ui.open(target.url)
    if open_error then
      report(open_error)
    end
  elseif target.kind == "file" or target.kind == "directory" then
    vim.cmd.edit(vim.fn.fnameescape(target.path))
  elseif target.kind == "anchor" then
    report("Heading navigation will be available after the backend update")
  end
  return true
end

function M.setup()
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
