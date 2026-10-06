local M = {}
local api = vim.api
local namespace = api.nvim_create_namespace("study.style")
local previous = {}

local function define_colors()
  local colors = require("study.config").options.colors
  for _, name in ipairs({ "Normal", "NormalNC" }) do
    local attributes = api.nvim_get_hl(0, { name = name, link = false })
    attributes.fg = colors.text
    api.nvim_set_hl(namespace, name, attributes)
  end
  for _, name in ipairs({
    "@markup.link",
    "@markup.link.label",
    "@markup.link.url",
    "@markup.link.markdown_inline",
    "@markup.link.label.markdown_inline",
    "@markup.link.url.markdown_inline",
    "markdownLink",
    "markdownLinkText",
    "markdownUrl",
    "markdownId",
    "markdownLinkDelimiter",
    "StudyDirectoryLink",
  }) do
    api.nvim_set_hl(namespace, name, { fg = colors.link, underline = true })
  end
  api.nvim_set_hl(namespace, "@markup.heading", { fg = colors.heading, bold = true })
  api.nvim_set_hl(namespace, "StudyDirectoryTitle", { fg = colors.heading, bold = true })
  for level = 1, 6 do
    for _, name in ipairs({ "@markup.heading." .. level .. ".markdown", "markdownH" .. level }) do
      api.nvim_set_hl(namespace, name, { fg = colors.heading, bold = true })
    end
  end
end

local function refresh()
  for _, window in ipairs(api.nvim_list_wins()) do
    local buffer = api.nvim_win_get_buf(window)
    local is_study = api.nvim_buf_get_name(buffer):match("%.study$")
      and vim.bo[buffer].filetype == "markdown"
    is_study = is_study or vim.bo[buffer].filetype == "study-directory"
    if is_study then
      if previous[window] == nil then
        local original = api.nvim_get_hl_ns({ winid = window })
        previous[window] = original == namespace and -1 or original
      end
      api.nvim_win_set_hl_ns(window, namespace)
    elseif previous[window] ~= nil then
      if api.nvim_get_hl_ns({ winid = window }) == namespace then
        api.nvim_win_set_hl_ns(window, previous[window])
      end
      previous[window] = nil
    end
  end
end

function M.setup()
  define_colors()
  local group = api.nvim_create_augroup("StudyStyle", { clear = true })
  api.nvim_create_autocmd({ "BufWinEnter", "WinEnter", "BufFilePost", "FileType" }, {
    group = group,
    callback = refresh,
  })
  api.nvim_create_autocmd("ColorScheme", {
    group = group,
    callback = function()
      define_colors()
      refresh()
    end,
  })
  api.nvim_create_autocmd("WinClosed", {
    group = group,
    callback = function(event)
      previous[tonumber(event.match)] = nil
    end,
  })
  refresh()
end

return M
