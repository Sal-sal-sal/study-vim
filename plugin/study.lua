if vim.g.loaded_study then
  return
end
vim.g.loaded_study = true
require("study").setup()
