local M = {}
local api = vim.api
local namespace = api.nvim_create_namespace("study.annotations")
local attached, versions, errors = {}, {}, {}

local function is_study(buffer)
  return api.nvim_buf_is_valid(buffer)
    and api.nvim_buf_is_loaded(buffer)
    and api.nvim_buf_get_name(buffer):match("%.study$")
    and vim.bo[buffer].filetype == "markdown"
end

local function render(buffer, annotations)
  api.nvim_buf_clear_namespace(buffer, namespace, 0, -1)
  for _, mark in ipairs(annotations) do
    if mark.kind == "note" then
      api.nvim_buf_set_extmark(buffer, namespace, mark.row, 0, {
        end_row = mark.row + 1,
        end_col = 0,
        hl_group = "StudyNote",
        hl_eol = true,
        priority = 150,
      })
      api.nvim_buf_set_extmark(buffer, namespace, mark.row, mark.start_col, {
        end_col = mark.start_col + 2,
        hl_group = "StudyNoteMarker",
        priority = 200,
      })
    end
  end
end

function M.refresh(buffer)
  if not api.nvim_buf_is_valid(buffer) then
    attached[buffer], versions[buffer], errors[buffer] = nil, nil, nil
    return
  end
  versions[buffer] = (versions[buffer] or 0) + 1
  local version = versions[buffer]
  if not is_study(buffer) then
    if api.nvim_buf_is_valid(buffer) then
      api.nvim_buf_clear_namespace(buffer, namespace, 0, -1)
    end
    return
  end
  if not attached[buffer] then
    attached[buffer] = api.nvim_buf_attach(buffer, false, {
      on_lines = function(_, changed)
        vim.schedule(function()
          M.refresh(changed)
        end)
      end,
      on_reload = function(_, changed)
        vim.schedule(function()
          M.refresh(changed)
        end)
      end,
      on_detach = function(_, detached)
        attached[detached], versions[detached], errors[detached] = nil, nil, nil
      end,
    })
  end
  vim.defer_fn(function()
    if versions[buffer] ~= version or not is_study(buffer) then
      return
    end
    local tick = api.nvim_buf_get_changedtick(buffer)
    require("study.backend").request_async({
      action = "annotations",
      text = table.concat(api.nvim_buf_get_lines(buffer, 0, -1, false), "\n"),
    }, function(annotations, error)
      if
        versions[buffer] ~= version
        or not is_study(buffer)
        or api.nvim_buf_get_changedtick(buffer) ~= tick
      then
        return
      end
      if error then
        api.nvim_buf_clear_namespace(buffer, namespace, 0, -1)
        if errors[buffer] ~= error then
          errors[buffer] = error
          vim.notify("study.nvim: " .. error, vim.log.levels.WARN)
        end
        return
      end
      errors[buffer] = nil
      render(buffer, annotations)
    end)
  end, 60)
end

local function refresh_all()
  for _, buffer in ipairs(api.nvim_list_bufs()) do
    M.refresh(buffer)
  end
end

function M.setup()
  local group = api.nvim_create_augroup("StudyAnnotations", { clear = true })
  api.nvim_create_autocmd({ "BufWinEnter", "FileType", "BufFilePost" }, {
    group = group,
    callback = function(event)
      M.refresh(event.buf)
    end,
  })
  api.nvim_create_autocmd("User", {
    group = group,
    pattern = "StudyBackendReady",
    callback = refresh_all,
  })
  refresh_all()
end

return M
