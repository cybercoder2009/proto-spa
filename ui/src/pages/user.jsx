import {
    AppstoreOutlined,
    DashboardOutlined,
    LogoutOutlined,
} from '@ant-design/icons';
import { Layout, Menu, Button } from 'antd';
import { Outlet, useLocation, useNavigate } from 'react-router-dom'
import { getUser, logout } from '../api';

const { Sider, Content, Header } = Layout;

const menuItems = [
    { key: '/user', icon: <DashboardOutlined />, label: 'Index' },
]

const User = _ => {
    const navigate = useNavigate()
    const location = useLocation()
    const user = getUser()

    const handleLogout = () => {
        logout();
        navigate('/login')
    };

    return (
        <Layout style={{ minHeight: '100vh' }}>
            <Sider collapsible>
                <div
                    style={{
                        height: 48,
                        margin: 16,
                        color: '#fff',
                        fontSize: 18,
                        fontWeight: 700,
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                    }}
                >
                    <AppstoreOutlined style={{ marginRight: 8 }} />
                    User
                </div>
                <Menu
                    theme="dark"
                    mode="inline"
                    selectedKeys={[location.pathname]}
                    items={menuItems}
                    onClick={({ key }) => navigate(key)}
                />
            </Sider>
            <Layout>
                <Header
                    style={{
                        background: '#fff',
                        padding: '0 24px',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'flex-end',
                        boxShadow: '0 1px 4px rgba(0, 21, 41, 0.08)',
                        zIndex: 1,
                    }}
                >
                    <div style={{ marginRight: 16, color: 'rgba(0, 0, 0, 0.65)' }}>
                        {user?.email}
                    </div>
                    <Button type="text" icon={<LogoutOutlined />} onClick={handleLogout}>
                        Logout
                    </Button>
                </Header>
                <Content
                    style={{
                        margin: 24,
                        background: '#fff',
                        padding: 24,
                        borderRadius: 8,
                        minHeight: 280,
                    }}
                >
                    <Outlet />
                </Content>
            </Layout>
        </Layout>
    )
}
export default User
