import { useState } from 'react'
import { Form, Input, Button, Card, message } from 'antd'
import { MailOutlined, LockOutlined } from '@ant-design/icons'
import { useNavigate } from 'react-router-dom'
import { login } from '../api'

const Login = _ => {
    const [loading, setLoading] = useState(false)
    const navigate = useNavigate()

    const handleSubmit = async (values) => {
        setLoading(true)
        try {
            const data = await login(values.email, values.password)
            message.success(`Welcome, ${data.email}`)
            navigate(data.role === 'user' ? '/user' : '/admin')
        } catch {
            message.error('Incorrect email or password')
        } finally {
            setLoading(false)
        }
    }

    return (
        <div
            style={{
                minHeight: '100vh',
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
                background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
            }}
        >
            <Card
                title="Login"
                style={{ width: 400, boxShadow: '0 8px 32px rgba(0,0,0,0.2)' }}
            >
                <Form
                    layout="vertical"
                    onFinish={handleSubmit}
                    initialValues={{
                        email: 'user@email.com',
                        password: 'password',
                    }}
                >
                    <Form.Item
                        name="email"
                        rules={[
                            { required: true, message: 'Please enter email' },
                            { type: 'email', message: 'Invalid email format' },
                        ]}
                    >
                        <Input prefix={<MailOutlined />} placeholder="Email" size="large" />
                    </Form.Item>

                    <Form.Item name="password" rules={[{ required: true, message: 'Please enter password' }]}>
                        <Input.Password prefix={<LockOutlined />} placeholder="Password" size="large" />
                    </Form.Item>

                    <Form.Item>
                        <Button
                            type="primary"
                            htmlType="submit"
                            block
                            size="large"
                            loading={loading}
                        >
                            LOGIN
                        </Button>
                    </Form.Item>
                </Form>
            </Card>
        </div>
    )
}
export default Login
