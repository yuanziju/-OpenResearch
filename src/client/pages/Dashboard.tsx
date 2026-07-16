// TODO: depends on agent-04 implementing API client for real data

interface StatCardProps {
  label: string
  value: number
}

const headingStyle: React.CSSProperties = {
  fontSize: '28px',
  color: '#e0e0e0',
  margin: '0 0 8px 0',
}

const subtitleStyle: React.CSSProperties = {
  fontSize: '16px',
  color: '#a0a0b0',
  margin: '0 0 32px 0',
}

const cardsRowStyle: React.CSSProperties = {
  display: 'flex',
  gap: '16px',
  flexWrap: 'wrap',
}

const cardStyle: React.CSSProperties = {
  background: '#16213e',
  padding: '20px',
  borderRadius: '8px',
  minWidth: '160px',
}

const cardLabelStyle: React.CSSProperties = {
  fontSize: '14px',
  color: '#a0a0b0',
  marginBottom: '8px',
}

const cardValueStyle: React.CSSProperties = {
  fontSize: '32px',
  fontWeight: 'bold',
  color: '#ffffff',
}

function StatCard({ label, value }: StatCardProps) {
  return (
    <div style={cardStyle}>
      <div style={cardLabelStyle}>{label}</div>
      <div style={cardValueStyle}>{value}</div>
    </div>
  )
}

function Dashboard() {
  return (
    <div>
      <h1 style={headingStyle}>Welcome to Open Research</h1>
      <p style={subtitleStyle}>
        An open-source research tool that redefines the research workflow.
      </p>
      <div style={cardsRowStyle}>
        <StatCard label="Total Papers" value={0} />
        <StatCard label="Recent Notes" value={0} />
        <StatCard label="Active Projects" value={0} />
      </div>
    </div>
  )
}

export default Dashboard
