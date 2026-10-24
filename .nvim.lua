local dap = require("dap")
local root = vim.fn.getcwd()
local elf = root .. "/target/thumbv4t-none-eabi/debug/gba-game"

dap.configurations.rust = {
	{
		name = "mgba + arm-none-eabi-gdb",
		type = "cppdbg",
		request = "launch",
		targetArchitecture = "arm",
		program = elf,
		cwd = root,
		args = {},
		stopAtEntry = false,
		externalConsole = false,
		MIMode = "gdb",
		miDebuggerPath = "arm-none-eabi-gdb",
		miDebuggerServerAddress = "localhost:2345",
		preLaunchTask = "cargo build",
		environment = { { name = "CARGO_TARGET_DIR", value = root .. "/target" } },
		setupCommands = {
			{ text = 'shell "mgba-qt" -g "' .. elf .. '" &' },
		},
	},
}
