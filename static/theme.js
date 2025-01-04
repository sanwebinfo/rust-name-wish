const toggle = document.getElementById('darkModeToggle');
const body = document.body;

function applyTheme(theme) {
    if (theme === 'dark') {
        body.classList.add('dark-mode');
        toggle.checked = true;
    } else {
        body.classList.remove('dark-mode');
        toggle.checked = false;
    }
}

function initTheme() {
    const savedTheme = localStorage.getItem('theme');
    if (savedTheme) {
        applyTheme(savedTheme);
    } else {
        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        applyTheme(prefersDark ? 'dark' : 'light');
    }
}

toggle.addEventListener('change', () => {
    const newTheme = toggle.checked ? 'dark' : 'light';
    applyTheme(newTheme);
    localStorage.setItem('theme', newTheme);
});

window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    const newSystemTheme = e.matches ? 'dark' : 'light';
    const savedTheme = localStorage.getItem('theme');
    if (!savedTheme) {
        applyTheme(newSystemTheme);
    }
});

window.addEventListener('load', initTheme);
