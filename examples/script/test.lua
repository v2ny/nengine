local r = 0
local g = 0
local b = 0
clear_window_color(r, g, b, 1.0)

print("HI! LUA")

setTimeout(function()
    print("This runs after 2 seconds")
	clear_window_color(0.9, 0, 0, 1.0)
end, 2000)