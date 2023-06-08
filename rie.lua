------------------------------------------
--[[ description

a script i used within golly to quickly compile a rie file *into clipboard*
and paste the output from the clipboard right into the Turing Machine's ROM

open "golly/Turing Machine.mc" or "golly/FlipIfElse.mc" (MC, not RIE)
then open/run this script through Golly itself
it should open up a terminal showing the compiler output
just press enter when it appears, or something

----------------------------------------]]
-- setup

local g = golly()

------------------------------------------
-- helpers

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
		return rect[1] + 4, rect[2] + 5
	end
end

------------------------------------------
-- begin

show "Which .rie file should I compile?" 
local rie = g.opendialog()
if #rie == 0 then
	g.exit("Cancelled.")
end

show "Finding Paste Location..." 
local x, y = rom_location()

show "Compiling..."
os.execute('cargo run --release "'..rie..'" --clip')

show "Pasting..."
g.paste(x, y, "or")

show "Pasted. Run pattern at 8^2 or faster."
