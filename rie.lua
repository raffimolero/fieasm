-- a script i used within golly to quickly compile a rie file *into clipboard*
-- and paste the output from the clipboard right into the Turing Machine's ROM

-- open golly/FlipIfElse.mc (MC, not RIE) or 
-- then open/run this script through Golly itself
-- it should open up a terminal showing the compiler output
-- just press enter when it appears, or something

------------------------------------------
-- setup

local g = golly()
local rie = g.opendialog()
if #rie == 0 then g.exit("Cancelled.") end

------------------------------------------
-- helpers

-- -- unused
-- local logs = ""
-- function log(thing)
-- 	logs = logs..tostring(thing)..' '
-- end

local function show(s)
	g.show(s)
	g.update()
end

------------------------------------------
-- functions

local function rom_location()
	g.run(1)
	local rect = g.getrect()
	g.reset()

	if #rect == 0 then
		show "Empty universe. Targeting x=0,y=0."
		return 0, 0
	else
		show "Pattern found. Assuming it is a turing machine without question."
		return { rect[1] + 4, rect[2] + 5 }
	end
end

------------------------------------------
-- begin

show "Finding Pattern..." 
local x, y = rom_location()

show "Compiling..."
os.execute('cargo run --release "'..rie..'" --clip')

show "Pasting..."
g.paste(x, y, "or")
show "Pasted. Run pattern at 8^2 or faster."

-- show(logs)
