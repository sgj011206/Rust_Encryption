
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
    command: 'cargo run -- help',
    output: 'ツールのヘルプ情報と利用可能なコマンド一覧を表示します。<br><br><span>Available commands:</span><br>&nbsp;&nbsp;encrypt&nbsp;&nbsp;decrypt&nbsp;&nbsp;keygen&nbsp;&nbsp;version'
  },
  encrypt: {
    command: 'cargo run -- encrypt file_path',
    output: '指定したファイルを読み込み、AES-256-GCM で暗号化します。<br><br><span>Input:</span>&nbsp;&nbsp;file_path<br><span>Result:</span>&nbsp;保護された暗号化ファイルを生成'
  },
  decrypt: {
    command: 'cargo run -- decrypt file_path',
    output: '対応する鍵を使用して指定したファイルを復号します。<br><br><span>Input:</span>&nbsp;&nbsp;encrypted file<br><span>Result:</span>&nbsp;元のファイル内容を復元'
  },
  keygen: {
    command: 'cargo run -- keygen',
    output: '新しい鍵ファイルを生成します。<br><br><span>Important:</span>&nbsp;鍵を安全に保管し、必ずバックアップを作成してください。'
  },
  version: {
    command: 'cargo run -- version',
    output: '現在の Rust Encryption のバージョン情報を表示します。<br><br><span>Status:</span>&nbsp;command completed successfully'
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

tabs.forEach((tab) => {
  tab.addEventListener('click', () => selectCommand(tab.dataset.command));
});

const copyButton = document.querySelector('.copy-command');

copyButton?.addEventListener('click', async () => {
  const text = commandCode?.textContent || '';

  try {
    await navigator.clipboard.writeText(text);

    const label = copyButton.querySelector('span');

    if (label) {
      label.textContent = 'コピーしました';
    }

    window.setTimeout(() => {
      if (label) {
        label.textContent = 'コピー';
      }
    }, 1300);
  } catch {
    const range = document.createRange();
    range.selectNodeContents(commandCode);

    const selection = window.getSelection();
    selection.removeAllRanges();
    selection.addRange(range);
  }
});

const observer = new IntersectionObserver(
  (entries) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        entry.target.classList.add('visible');
        observer.unobserve(entry.target);
      }
    });
  },
  { threshold: 0.12 }
);

document.querySelectorAll('.reveal').forEach((el, index) => {
  el.style.transitionDelay = `${Math.min(index % 4, 3) * 70}ms`;
  observer.observe(el);
});

