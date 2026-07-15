const header = document.querySelector('.site-header');
const menuButton = document.querySelector('.menu-toggle');
const nav = document.querySelector('.primary-nav');

const updateHeader = () => {
  header.classList.toggle('scrolled', window.scrollY > 18);
};
updateHeader();
window.addEventListener('scroll', updateHeader, { passive: true });

menuButton?.addEventListener('click', () => {
  const open = menuButton.getAttribute('aria-expanded') === 'true';
  menuButton.setAttribute('aria-expanded', String(!open));
  nav.classList.toggle('open', !open);
  document.body.style.overflow = open ? '' : 'hidden';
});

nav?.querySelectorAll('a').forEach((link) => {
  link.addEventListener('click', () => {
    menuButton?.setAttribute('aria-expanded', 'false');
    nav.classList.remove('open');
    document.body.style.overflow = '';
  });
});

const commandData = {
  help: {
    command: 'help',
    output: '显示工具帮助信息与可用命令列表。<br><br><span>Available commands:</span><br>&nbsp;&nbsp;encrypt&nbsp;&nbsp;decrypt&nbsp;&nbsp;keygen&nbsp;&nbsp;version'
  },
  encrypt: {
    command: 'encrypt file_path',
    output: '读取指定文件并执行 AES-256-GCM 加密。<br><br><span>Input:</span>&nbsp;&nbsp;file_path<br><span>Result:</span>&nbsp;生成受保护的加密文件'
  },
  decrypt: {
    command: 'decrypt file_path',
    output: '使用对应密钥解密指定文件。<br><br><span>Input:</span>&nbsp;&nbsp;encrypted file<br><span>Result:</span>&nbsp;恢复原始文件内容'
  },
  keygen: {
    command: 'keygen',
    output: '生成新的随机密钥或密钥文件。<br><br><span>Important:</span>&nbsp;请将密钥安全保存并建立备份。'
  },
  version: {
    command: 'version',
    output: '显示当前 Rust Encryption 的版本信息。<br><br><span>Status:</span>&nbsp;command completed successfully'
  }
};

const commandCode = document.getElementById('command-code');
const terminalOutput = document.getElementById('terminal-output');
const tabs = [...document.querySelectorAll('.command-tab')];

function selectCommand(name) {
  const data = commandData[name];
  if (!data || !commandCode || !terminalOutput) return;
  terminalOutput.classList.add('changing');
  window.setTimeout(() => {
    commandCode.textContent = data.command;
    terminalOutput.innerHTML = data.output;
    terminalOutput.classList.remove('changing');
  }, 150);
  tabs.forEach((tab) => {
    const active = tab.dataset.command === name;
    tab.classList.toggle('active', active);
    tab.setAttribute('aria-selected', String(active));
  });
}

tabs.forEach((tab) => tab.addEventListener('click', () => selectCommand(tab.dataset.command)));

const copyButton = document.querySelector('.copy-command');
copyButton?.addEventListener('click', async () => {
  const text = commandCode?.textContent || '';
  try {
    await navigator.clipboard.writeText(text);
    const label = copyButton.querySelector('span');
    if (label) label.textContent = '已复制';
    window.setTimeout(() => { if (label) label.textContent = '复制'; }, 1300);
  } catch {
    const range = document.createRange();
    range.selectNodeContents(commandCode);
    const selection = window.getSelection();
    selection.removeAllRanges();
    selection.addRange(range);
  }
});

const observer = new IntersectionObserver((entries) => {
  entries.forEach((entry) => {
    if (entry.isIntersecting) {
      entry.target.classList.add('visible');
      observer.unobserve(entry.target);
    }
  });
}, { threshold: 0.12 });

document.querySelectorAll('.reveal').forEach((el, index) => {
  el.style.transitionDelay = `${Math.min(index % 4, 3) * 70}ms`;
  observer.observe(el);
});
