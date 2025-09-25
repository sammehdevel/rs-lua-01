a = {} -- create a table
k = "x"
a[k] = 10        -- new entry, with key="x" and value=10
a[20] = "great"  -- new entry, with key=20 and value="great"
print(a["x"])
k = 20
print(a[k])


-- Each table may store values with different types of indices and it grows as it needs to accommodate new entries:
a = {}     -- empty table
-- create 1000 new entries
for i=1,1000 do a[i] = i*2 end
print(a[9])    --> 18
a["x"] = 10
print(a["x"])  --> 10
print(a["y"])  --> nil

-- Arrays are just tables with integer as indices
a = {}
for i = 1,10 do a[i] = i end
print(a[10])

-- print the table
print("print a table")
a = {}
for i = 1,10 do a[i] = i end
for i,line in ipairs(a) do
    print(i.. "," ..line)
end
-- Another way to iterate a table
local i = 1
while a[i] do
    print(i .. ","..a[i])
    i = i + 1
end

-- Initialize a table
print("initialize a table")
a = {1,2,3,4,5,6,7,8,9,10}
for i,line in ipairs(a) do
    print(i.. "," ..line)
end
b = {x=1, y=1, ["z"]=2}
for i,line in pairs(b) do -- Use "pairs" not "ipairs" here
    print(i.. "," ..line)
end
