import { ACADEMIC_VENUES } from '@/shared/constants'

const headingStyle: React.CSSProperties = {
  fontSize: '28px',
  color: '#e0e0e0',
  margin: '0 0 24px 0',
}

const searchBarStyle: React.CSSProperties = {
  display: 'flex',
  gap: '8px',
  marginBottom: '24px',
}

const searchInputStyle: React.CSSProperties = {
  flex: 1,
  padding: '10px 14px',
  fontSize: '14px',
  borderRadius: '6px',
  border: '1px solid #2a2a4a',
  background: '#16213e',
  color: '#e0e0e0',
  boxSizing: 'border-box',
}

const searchButtonStyle: React.CSSProperties = {
  padding: '10px 20px',
  fontSize: '14px',
  borderRadius: '6px',
  border: 'none',
  background: '#e94560',
  color: '#ffffff',
  cursor: 'pointer',
  fontWeight: 'bold',
}

const filterSectionStyle: React.CSSProperties = {
  background: '#16213e',
  padding: '16px',
  borderRadius: '8px',
  marginBottom: '24px',
  display: 'flex',
  gap: '16px',
  flexWrap: 'wrap',
  alignItems: 'flex-end',
}

const filterGroupStyle: React.CSSProperties = {
  display: 'flex',
  flexDirection: 'column',
  gap: '4px',
}

const filterLabelStyle: React.CSSProperties = {
  fontSize: '12px',
  color: '#a0a0b0',
}

const inputStyle: React.CSSProperties = {
  padding: '8px 10px',
  fontSize: '14px',
  borderRadius: '6px',
  border: '1px solid #2a2a4a',
  background: '#1a1a2e',
  color: '#e0e0e0',
  boxSizing: 'border-box',
}

const emptyStateStyle: React.CSSProperties = {
  display: 'flex',
  justifyContent: 'center',
  alignItems: 'center',
  minHeight: '300px',
  color: '#a0a0b0',
  fontSize: '16px',
}

function Search() {
  return (
    <div>
      <h1 style={headingStyle}>Search</h1>
      <div style={searchBarStyle}>
        <input
          type="text"
          placeholder="Search for papers..."
          style={searchInputStyle}
        />
        <button type="button" style={searchButtonStyle}>
          Search
        </button>
      </div>
      <div style={filterSectionStyle}>
        <div style={filterGroupStyle}>
          <label style={filterLabelStyle}>Start Date</label>
          <input type="date" style={inputStyle} />
        </div>
        <div style={filterGroupStyle}>
          <label style={filterLabelStyle}>End Date</label>
          <input type="date" style={inputStyle} />
        </div>
        <div style={filterGroupStyle}>
          <label style={filterLabelStyle}>Venue</label>
          <select style={inputStyle} defaultValue="">
            <option value="" disabled>
              Select a venue
            </option>
            {ACADEMIC_VENUES.map((venue) => (
              <option key={venue} value={venue}>
                {venue}
              </option>
            ))}
          </select>
        </div>
      </div>
      <div style={emptyStateStyle}>
        Search for papers across academic databases.
      </div>
    </div>
  )
}

export default Search
