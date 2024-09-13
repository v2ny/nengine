local r = 0.0
local g = 0.4
local b = 0.8
if not os.execute("cls") then os.execute("clear") end
clear_window_color(r, g, b, 1.0)
print("Changed window color to:", r,g,b);