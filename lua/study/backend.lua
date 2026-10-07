local M = {}
local config = require("study.config")
local source = debug.getinfo(1, "S").source:sub(2)
M.root = vim.fs.dirname(vim.fs.dirname(vim.fs.dirname(source)))

function M.binary()
  if config.options.binary then
    return vim.fn.expand(config.options.binary)
  end
  local name = "study" .. (vim.fn.has("win32") == 1 and ".exe" or "")
  local compiled = vim.fs.joinpath(M.root, "target", "release", name)
  if vim.fn.executable(compiled) == 1 then
    return compiled
  end
  if vim.fn.executable(name) == 1 then
    return name
  end
  return nil
end

local function decode(result)
  if result.code ~= 0 then
    return nil, result.stderr ~= "" and result.stderr or "Rust request failed or timed out"
  end
  local decoded, response = pcall(vim.json.decode, result.stdout)
  if not decoded or type(response) ~= "table" or type(response.ok) ~= "boolean" then
    return nil, "Rust backend returned an invalid response"
  end
  if not response.ok then
    return nil, response.error
  end
  return response.data
end

function M.request(request)
  local executable = M.binary()
  if not executable then
    return nil, "Rust backend is missing. Run :StudyBuild or cargo install --path . --locked."
  end
  local success, result = pcall(function()
    return vim
      .system({ executable, "--request" }, {
        text = true,
        stdin = vim.json.encode(request),
      })
      :wait(config.options.timeout)
  end)
  if not success then
    return nil, tostring(result)
  end
  return decode(result)
end

function M.request_async(request, callback)
  local executable = M.binary()
  if not executable then
    callback(nil, "Rust backend is missing. Run :StudyBuild or cargo install --path . --locked.")
    return
  end
  local success, error = pcall(
    vim.system,
    { executable, "--request" },
    {
      text = true,
      stdin = vim.json.encode(request),
      timeout = config.options.timeout,
    },
    vim.schedule_wrap(function(result)
      callback(decode(result))
    end)
  )
  if not success then
    callback(nil, tostring(error))
  end
end

function M.build()
  vim.notify("study.nvim: building the Rust backend")
  local success, error = pcall(
    vim.system,
    { "cargo", "build", "--release", "--locked" },
    {
      cwd = M.root,
      text = true,
    },
    vim.schedule_wrap(function(result)
      if result.code == 0 then
        vim.notify("study.nvim: backend is ready")
        vim.api.nvim_exec_autocmds("User", { pattern = "StudyBackendReady" })
      else
        vim.notify("study.nvim: " .. result.stderr, vim.log.levels.ERROR)
      end
    end)
  )
  if not success then
    vim.notify("study.nvim: " .. tostring(error), vim.log.levels.ERROR)
  end
end

return M
