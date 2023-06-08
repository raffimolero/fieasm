# rieasm

A program that converts `.rie` files into RLEs to paste into Golly.

---

<details>
<summary>Instructions</summary>

(outdated summary)

1. make sure you have [Golly](https://sourceforge.net/projects/golly/files/) and [Rust](https://www.rust-lang.org/tools/install) installed.

2. Clone this repo.

3. Copy the contents of `golly/Flow6.rule` as text and paste it into Golly.

4. Open up `golly/Turing Machine.mc`. It should look something like this:
    <details>
    <summary>(Show Image)</summary>

    | ![image](https://user-images.githubusercontent.com/49224759/169102690-671830f1-47ef-4f2c-a58e-61fc94749c04.png) |
    |:--:|
    | The Flow6 Turing Machine. |
    
    </details>

    ---

5. Go back to your terminal and type `cargo run program --clip`. This will first compile the rieasm assembler, which will then run, compiling the `program.rie` file found in this repository into your clipboard.
    <details>
    <summary>(Show Image)</summary>

    | ![image](https://user-images.githubusercontent.com/49224759/169112146-7200754b-b4c9-4317-bfb1-db79eb675fb6.png) |
    |:--:|
    | Terminal should look something like this after running the command. |
    
    </details>
    
    ---


6. Go back to Golly, and follow these instructions to paste the RLE:
    <details>
    <summary>(Show Image)</summary>

    | ![image](https://user-images.githubusercontent.com/49224759/169105264-ee759a54-9f00-42d0-9187-622c06228fb4.png) |
    |:--:|
    | Hover and scroll to zoom. No need to hold any buttons. |
    
    </details>
    
    ---

    <details>
    <summary>(Show Image)</summary>

    | ![image](https://user-images.githubusercontent.com/49224759/169109031-2c85e079-807b-443d-beeb-13ed224b257a.png) |
    |:--:|
    | Align your crosshair with the tile marked with green. It should say `XY=0 0` on the top bar. |
    | Once the cursor is aligned, hit Ctrl+V and left click. |
    
    </details>
    
    ---

7. Set the step size. Press the `+` and `-` keys on your keyboard until it's the right speed.
    <details>
    <summary>(Show Image)</summary>

    | ![image](https://user-images.githubusercontent.com/49224759/169106753-1dfaa5f7-6b77-4293-a4a0-f0155a62a35b.png) |
    |:--:|
    | I recommend `8^0` if you want to track the exact paths of the signals. |
    | `8^1` if you want to see individual register operations. |
    | `8^2` if you want to see what the whole program does. |
    
    </details>
    
    ---

8. Run the simulation by clicking the Green Play button on the top left.
    <details>
    <summary>(Show Image)</summary>

    | ![image](https://user-images.githubusercontent.com/49224759/169111090-f57aa923-8391-41a2-b970-225990f00878.png) |
    |:--:|
    | ![image](https://user-images.githubusercontent.com/49224759/169110580-d5408b94-1b32-4017-97a1-443d71b454b9.png) |
    | The red button will pause the simulation, while the blue button will reset it. |
    
    </details>
    
    ---

</details>

---

<details>
<summary>File format</summary>
Lines of code are just tabs followed by tokens. It's pretty much like a `.csv` file but with tabs instead of commas.

If a line doesn't begin with a tab, it is a comment.

example comment:
```
cheese elephant
```

A tab width of 8 is recommended, but feel free to use tab width 4 if you like working with badly aligned instructions, or spaces if you want the program to refuse to compile :v

---

**Note: For display consistency, I will be using spaces in the code.**

The first real line of `.rie` code inside of the file is a header and must look like this:
```
        STATE   ARG     GOTO    READ    REG     REG     REG     ARM
```

with 0 or more `reg` columns. In this case, 3.
The number of `reg` columns will specify how many registers the **target** Turing Machine model has.
the `ARM` column is optional.

The provided Turing Machine file has exactly **4 bits of state and 3 registers,** so unless you know how to mod the machine, **keep it at 3 registers.**

As mentioned earlier for all `.rie` code, every token is preceded with tabs, as in `<tab>STATE<tab>ARG<tab>GOTO<tab>READ` and so on.

Every valid line after the header must now stick to the format specified by the header:

- `STATE` is required. It must be a nonnegative integer, and is what `GOTO` looks for when jumping to the next instruction.
- `ARG` is also required. It must be either `true` or `false`, is always paired with state, and is where `READ` and `REG '?'` go to when finding the next instruction.
- `GOTO` jumps to the corresponding `STATE` after the current instruction finishes. defaults to the same value as `STATE`.
- `READ` returns either `true` or `false` as the next `arg`.
- `REG` instructions may either be omitted, or one of the following:
    - `>` Push register head one step to the right.
    - `<` Pull register head one step to the left.
    - `%` Flip the bit at the register head.
    - `%<` Executes `%` then `<`. This is the only "combination" of instructions.<sup>[1]</sup>
    - `?` Reads the bit at the register head. This will become the next `ARG`.
- `ARM` instructions can be one of `2^4=16` combinations of the following, so long as they follow the order `<>C?`:
    1. `<` Send a 2-long signal. Equivalent to `%<` for `REG`.
    2. `>` Send a 1-long signal. The same instruction as in `REG`.
    3. `C` Send a 3-long signal. Bends the construction arm. If `>` is already specified, `C` acts like `<`.
    4. `?` Send a read signal to the arm. The same instruction as in `REG`.
- anything after `ARM` (or the last `REG`, if there is no arm,) is a comment.

There may only be at most one read. Having no reads will end the program.

Notes
-
<sup>[1] Actually, `%<` is the more basic instruction. It just so happens that the register "drivers" allow chaining `%<` together with `>`, making `%<>` which is just `%`.

</details>

---
