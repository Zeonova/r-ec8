import init, * as wasm from "../wasm.js";

const WIDTH = 64;
const HEIGHT = 32;
const SCALE = 15;
const TICKS_PER_FRAME = 10;
let anim_frame = 0;

const canvas = document.getElementById("canvas");
canvas.width = WIDTH * SCALE;
canvas.height = HEIGHT * SCALE;

const ctx = canvas.getContext("2d");
ctx.fillStyle = "black";
ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);

const input = document.getElementById("fileinput");

console.log("Hello...!");
async function run() {
  await init();
  let chip8 = new wasm.EmuWasm();
  if (!chip8) {
    console.error("chip8 is not initialized properly.");
    return;
  }

  document.addEventListener("keydown", function (evt) {
    chip8.keypress(evt, true);
  });
  document.addEventListener("keyup", function (evt) {
    chip8.keypress(evt, false);
  });
  input.addEventListener(
    "change",
    function (evt) {
      if (anim_frame != 0) {
        window.cancelAnimationFrame(anim_frame);
      }
      let file = evt.target.files[0];
      if (!file) {
        alert("No file selected. Please choose a file.");

        return;
      }
      let fr = new FileReader();
      fr.onload = function (e) {
        console.log("js loading...!");
        let buffer = fr.result;
        const rom = new Uint8Array(buffer);
        // 检查 rom 的内容
        console.log("ROM Data: ", rom);

        // 检查 ROM 是否为空
        if (rom.length === 0) {
          console.error("The ROM file is empty.");
          return;
        }

        // 检查 ROM 前几个字节的内容，确保数据读取正常
        console.log("First few bytes of ROM: ", rom.slice(0, 10));
        chip8.reset();

        try {
          chip8.load_game(rom);
        } catch (e) {
          console.error("Error calling load_game:", e);
        }
        mainloop(chip8);
      };
      // 错误处理：如果文件读取失败
      fr.onerror = function () {
        alert("Failed to read the file. Please try again.");
      };
      fr.readAsArrayBuffer(file);
    },
    false,
  );
  function mainloop(chip8) {
    for (let i = 0; i < TICKS_PER_FRAME; i++) {
      chip8.tick();
    }
    chip8.tick_timers();

    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);
    ctx.fillStyle = "white";
    chip8.draw_screen(SCALE);

    anim_frame = window.requestAnimationFrame(() => {
      mainloop(chip8);
    });
  }
}

run().catch(console.error);
