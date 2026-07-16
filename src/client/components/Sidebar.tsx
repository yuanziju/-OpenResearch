import { NavLink } from 'react-router-dom'

interface NavItem {
  to: string
  label: string
  icon: string
}

const navItems: NavItem[] = [
  { to: '/', label: 'Dashboard', icon: '🏠' },
  { to: '/papers', label: 'Papers', icon: '📄' },
  { to: '/search', label: 'Search', icon: '🔍' },
  { to: '/notes', label: 'Notes', icon: '📝' },
  { to: '/projects', label: 'Projects', icon: '📁' },
  { to: '/settings', label: 'Settings', icon: '⚙️' },
]

const sidebarStyle: React.CSSProperties = {
  width: '240px',
  background: '#16213e',
  padding: '16px 0',
  boxSizing: 'border-box',
  minHeight: '100vh',
}

const titleStyle: React.CSSProperties = {
  fontWeight: 'bold',
  fontSize: '18px',
  color: '#e0e0e0',
  padding: '16px',
  margin: 0,
}

const baseLinkStyle: React.CSSProperties = {
  display: 'block',
  padding: '12px 16px',
  color: '#a0a0b0',
  textDecoration: 'none',
  boxSizing: 'border-box',
  borderLeft: '3px solid transparent',
}

const activeLinkStyle: React.CSSProperties = {
  background: '#0f3460',
  color: '#ffffff',
  borderLeft: '3px solid #e94560',
}

function getLinkStyle(isActive: boolean): React.CSSProperties {
  if (isActive) {
    return { ...baseLinkStyle, ...activeLinkStyle }
  }
  return baseLinkStyle
}

function Sidebar() {
  return (
    <nav style={sidebarStyle}>
      <h1 style={titleStyle}>Open Research</h1>
      {navItems.map((item) => (
        <NavLink
          key={item.to}
          to={item.to}
          end={item.to === '/'}
          style={({ isActive }) => getLinkStyle(isActive)}
        >
          <span style={{ marginRight: '8px' }}>{item.icon}</span>
          {item.label}
        </NavLink>
      ))}
    </nav>
  )
}

export default Sidebar
