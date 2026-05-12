const BASE = '';

function getToken() {
    return localStorage.getItem('token');
}

export function setToken(token) {
    if (token) {
        localStorage.setItem('token', token);
    } else {
        localStorage.removeItem('token');
    }
}

export function getUser() {
    try {
        const raw = localStorage.getItem('user');
        return raw ? JSON.parse(raw) : null;
    } catch {
        return null;
    }
}

export function setUser(user) {
    if (user) {
        localStorage.setItem('user', JSON.stringify(user));
    } else {
        localStorage.removeItem('user');
    }
}

export async function request(path, options = {}) {
    const token = getToken();
    const headers = { 'Content-Type': 'application/json', ...options.headers };
    if (token) {
        headers['Authorization'] = `Bearer ${token}`;
    }
    const res = await fetch(`${BASE}${path}`, { headers, ...options });
    if (res.status === 401) {
        setToken(null);
        setUser(null);
        window.location.href = '/login';
        throw new Error('Unauthorized');
    }
    if (!res.ok) {
        const body = await res.text();
        throw new Error(`HTTP ${res.status}: ${body}`);
    }
    if (res.status === 204) return null;
    return res.json();
}

// ---- Auth ----
export const login = async (email, password) => {
    const data = await request('/api/login', {
        method: 'POST',
        body: JSON.stringify({ email, password }),
    });
    setToken(data.token);
    setUser({ email: data.email, role: data.role });
    return data;
}

export const logout = () => {
    setToken(null)
    setUser(null)
}