const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

function caesarCipher(str, shift) {
  return str
    .split('')
    .map(char => {
      if (char.match(/[a-zA-Z]/)) {
        const code = char.charCodeAt(0);
        const isUpperCase = char === char.toUpperCase();
        const offset = isUpperCase ? 65 : 97;
        return String.fromCharCode((code - offset + shift) % 26 + offset);
      } else {
        return char;
      }
    })
    .join('');
}

// 让 process.stdin 发出 'keypress' 事件
readline.emitKeypressEvents(process.stdin);
// 设置输入流的原始模式
rl.setRawMode(true);

rl.question('请输入要加密/解密的文本（输入 cmd + enter 结束）: ', (text) => {
  let lines = [];
  lines.push(text);
  // 监听 'keypress' 事件
  process.stdin.on('keypress', (ch, key) => {
    // 如果按下 cmd + enter，写入一个换行符，触发 'line' 事件
    if (key && key.name === 'return' && key.meta) {
      rl.write('\n');
    }
  });
  rl.on('line', (line) => {
    // 如果输入为空，表示结束输入
    if (line === '') {
      // 暂停 'keypress' 事件的监听
      rl.pause();
      rl.question('请输入偏移量（正数为加密，负数为解密）: ', (shift) => {
        const encryptedText = caesarCipher(lines.join('\n'), parseInt(shift, 10));
        console.log('加密/解密结果:', encryptedText);
        rl.close();
      });
    } else {
      lines.push(line);
    }
  });
});
