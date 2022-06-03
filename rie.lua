-- a script i used within golly to quickly compile a rie file *into clipboard*
-- and paste the output from the clipboard right into the Turing Machine's ROM

------------------------------------------
-- setup

-- this is the path to the rie file, change it to whatever
local rie = "golly/FlipIfElse.rie"

-- then open golly/FlipIfElse.mc (MC, not RIE)
-- then open/run this script through Golly itself
-- it should open up a terminal showing the compiler output
-- just press enter when it appears, or something

local g = golly()

------------------------------------------
-- helpers

local logs = ""
function log(thing)
	logs = logs..tostring(thing)..' '
end

local function show(s)
	g.show(s)
	g.update()
end

------------------------------------------
-- functions

local function rom_location()
	g.run(1)
	local tx, ty = table.unpack(g.getrect())
	g.reset()
	return tx + 4, ty + 5
end

------------------------------------------
-- begin

show "Compiling..."
os.execute("cargo run --release "..rie.." --clip")

show "Finding Pattern..." 
local x, y = rom_location()
show "Pattern found. Assuming it is a turing machine without question."

show "Pasting..."
g.paste(x, y, "or")
show "Pasted. Run pattern at 8^2 or faster."

-- show(logs)
