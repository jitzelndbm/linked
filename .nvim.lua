require("lspconfig").nil_ls.setup({})
require("lspconfig").rust_analyzer.setup({})

vim.api.nvim_create_autocmd({ "BufRead", "BufNewFile" }, {
	pattern = "*.html",
	command = "set filetype=htmldjango",
})
