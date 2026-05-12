import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { ConfigProvider, App as AntApp } from 'antd'
import enUS from 'antd/locale/en_US'
import { HashRouter, Routes, Route, Navigate } from 'react-router-dom'

import './index.css'
import Admin from './pages/admin'
import AdminIndex from './pages/admin_index'
import User from './pages/user'
import UserIndex from './pages/user_index'
import Login from './pages/login'
import { getUser } from './api'

const RequireAuth = ({ children }) => {
    const user = getUser()
    if (!user) {
        return <Navigate to="/login" replace />
    }
    return children
};

const App = _ => {
    return (
        <ConfigProvider locale={enUS}>
            <AntApp>
                <HashRouter>
                    <Routes>
                        <Route path="/login" element={<Login />} />
                        <Route
                            path="/admin"
                            element={
                                <RequireAuth>
                                    <Admin />
                                </RequireAuth>
                            }
                        >
                            <Route index element={<AdminIndex />} />
                        </Route>
                        <Route
                            path="/user"
                            element={
                                <RequireAuth>
                                    <User />
                                </RequireAuth>
                            }
                        >
                            <Route index element={<UserIndex />} />
                        </Route>
                        <Route path="*" element={<Navigate to="/login" replace />} />
                    </Routes>
                </HashRouter>
            </AntApp>
        </ConfigProvider>
    );
}

createRoot(document.getElementById('root')).render(
    <StrictMode>
        <App />
    </StrictMode>
);
