local M = {}
local api = vim.api
local attached = {}

local function refresh()
  local key = require("study.config").options.mappings.open
  for _, buffer in ipairs(api.nvim_list_bufs()) do
    local is_study = api.nvim_buf_get_name(buffer):match("%.study$")
      and vim.bo[buffer].filetype == "markdown"
    if attached[buffer] and (not is_study or attached[buffer] ~= key) then
      pcall(vim.keymap.del, "n", attached[buffer], { buffer = buffer })
      attached[buffer] = nil
    end
    if is_study and key and not attached[buffer] then
      vim.keymap.set("n", key, function()
        if not require("study.commands").open(false) then
          vim.cmd.normal({
            args = { api.nvim_replace_termcodes("<CR>", true, false, true) },
            bang = true,
          })
        end
      end, { buffer = buffer, desc = "Follow study link" })
      attached[buffer] = key
    end
  end
end

function M.setup()
  local group = api.nvim_create_augroup("StudyBuffers", { clear = true })
  api.nvim_create_autocmd(
    { "BufWinEnter", "FileType", "BufFilePost" },
    { group = group, callback = refresh }
  )
  api.nvim_create_autocmd("BufWipeout", {
    group = group,
    callback = function(event)
      attached[event.buf] = nil
    end,
  })
  refresh()
end

return M
