import { ReactNode } from 'react'
import Sidebar from './Sidebar'

interface LayoutProps {
  children?: ReactNode
}

const layoutStyle: React.CSSProperties = {
  display: 'flex',
  minHeight: '100vh',
  background: '#1a1a2e',
}

const contentStyle: React.CSSProperties = {
  flex: 1,
  padding: '24px',
  overflow: 'auto',
}

function Layout({ children }: LayoutProps) {
  return (
    <div style={layoutStyle}>
      <Sidebar />
      <div style={contentStyle}>{children}</div>
    </div>
  )
}

export default Layout
