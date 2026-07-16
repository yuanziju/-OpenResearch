import { useEffect, useState } from 'react'
import { ACADEMIC_VENUES } from '@/shared/constants'
import { useSearchStore } from '@/client/state'
import type { Paper } from '@/shared/types'

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

const secondaryButtonStyle: React.CSSProperties = {
  padding: '10px 16px',
  fontSize: '14px',
  borderRadius: '6px',
  border: '1px solid #2a2a4a',
  background: 'transparent',
  color: '#a0a0b0',
  cursor: 'pointer',
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

const resultsListStyle: React.CSSProperties = {
  display: 'flex',
  flexDirection: 'column',
  gap: '8px',
}

const resultCardStyle: React.CSSProperties = {
  background: '#16213e',
  padding: '16px',
  borderRadius: '8px',
}

const resultTitleStyle: React.CSSProperties = {
  fontSize: '16px',
  fontWeight: 'bold',
  color: '#e0e0e0',
  margin: '0 0 4px 0',
}

const resultMetaStyle: React.CSSProperties = {
  fontSize: '13px',
  color: '#a0a0b0',
  margin: 0,
}

const emptyStateStyle: React.CSSProperties = {
  display: 'flex',
  justifyContent: 'center',
  alignItems: 'center',
  minHeight: '300px',
  color: '#a0a0b0',
  fontSize: '16px',
}

const messageStyle: React.CSSProperties = {
  padding: '10px 14px',
  borderRadius: '6px',
  marginBottom: '16px',
  fontSize: '14px',
}

const metaRowStyle: React.CSSProperties = {
  fontSize: '13px',
  color: '#a0a0b0',
  margin: '8px 0 16px 0',
}

function Search() {
  const query = useSearchStore((s) => s.query)
  const results = useSearchStore((s) => s.results)
  const total = useSearchStore((s) => s.total)
  const sources = useSearchStore((s) => s.sources)
  const loading = useSearchStore((s) => s.loading)
  const error = useSearchStore((s) => s.error)
  const hasSearched = useSearchStore((s) => s.hasSearched)
  const savedQueries = useSearchStore((s) => s.savedQueries)
  const setQuery = useSearchStore((s) => s.setQuery)
  const setFilters = useSearchStore((s) => s.setFilters)
  const executeSearch = useSearchStore((s) => s.executeSearch)
  const fetchSavedQueries = useSearchStore((s) => s.fetchSavedQueries)
  const saveCurrentQuery = useSearchStore((s) => s.saveCurrentQuery)
  const deleteSavedQuery = useSearchStore((s) => s.deleteSavedQuery)

  const [startDate, setStartDate] = useState('')
  const [endDate, setEndDate] = useState('')
  const [venue, setVenue] = useState('')

  useEffect(() => {
    fetchSavedQueries()
  }, [fetchSavedQueries])

  const handleSearch = () => {
    const dateRange =
      startDate || endDate
        ? { start: startDate || '', end: endDate || '' }
        : undefined
    const venues = venue ? [venue] : undefined
    setFilters({ dateRange, venues })
    void executeSearch()
  }

  const handleSave = async () => {
    await saveCurrentQuery()
  }

  const handleDeleteSaved = async (id: string) => {
    await deleteSavedQuery(id)
  }

  return (
    <div>
      <h1 style={headingStyle}>Search</h1>
      <div style={searchBarStyle}>
        <input
          type="text"
          placeholder="Search for papers..."
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === 'Enter') handleSearch()
          }}
          style={searchInputStyle}
        />
        <button type="button" style={searchButtonStyle} onClick={handleSearch}>
          Search
        </button>
        <button type="button" style={secondaryButtonStyle} onClick={handleSave} disabled={!query}>
          Save query
        </button>
      </div>
      <div style={filterSectionStyle}>
        <div style={filterGroupStyle}>
          <label style={filterLabelStyle}>Start Date</label>
          <input
            type="date"
            value={startDate}
            onChange={(e) => setStartDate(e.target.value)}
            style={inputStyle}
          />
        </div>
        <div style={filterGroupStyle}>
          <label style={filterLabelStyle}>End Date</label>
          <input
            type="date"
            value={endDate}
            onChange={(e) => setEndDate(e.target.value)}
            style={inputStyle}
          />
        </div>
        <div style={filterGroupStyle}>
          <label style={filterLabelStyle}>Venue</label>
          <select
            style={inputStyle}
            value={venue}
            onChange={(e) => setVenue(e.target.value)}
          >
            <option value="">Any venue</option>
            {ACADEMIC_VENUES.map((v) => (
              <option key={v} value={v}>
                {v}
              </option>
            ))}
          </select>
        </div>
      </div>

      {error && (
        <div style={{ ...messageStyle, color: '#e94560', background: '#2a1a2a' }}>{error}</div>
      )}

      {savedQueries.length > 0 && (
        <div style={{ marginBottom: '24px' }}>
          <div style={{ fontSize: '13px', color: '#a0a0b0', marginBottom: '8px' }}>
            Saved queries
          </div>
          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '8px' }}>
            {savedQueries.map((sq) => (
              <div
                key={sq.id}
                style={{
                  background: '#1a1a2e',
                  border: '1px solid #2a2a4a',
                  borderRadius: '6px',
                  padding: '6px 10px',
                  fontSize: '13px',
                  color: '#e0e0e0',
                  display: 'flex',
                  alignItems: 'center',
                  gap: '8px',
                }}
              >
                <span>{sq.query || '(no query text)'}</span>
                <button
                  type="button"
                  onClick={() => handleDeleteSaved(sq.id)}
                  style={{
                    background: 'transparent',
                    border: 'none',
                    color: '#e94560',
                    cursor: 'pointer',
                    padding: 0,
                  }}
                  aria-label="Delete saved query"
                >
                  ×
                </button>
              </div>
            ))}
          </div>
        </div>
      )}

      {loading ? (
        <div style={emptyStateStyle}>Searching…</div>
      ) : results.length === 0 ? (
        <div style={emptyStateStyle}>
          {hasSearched
            ? 'No results. Try a different query or adjust filters.'
            : 'Search for papers across academic databases.'}
        </div>
      ) : (
        <div>
          <div style={metaRowStyle}>
            {total} result{total === 1 ? '' : 's'}
            {sources.length > 0 ? ` from ${sources.join(', ')}` : ''}
          </div>
          <div style={resultsListStyle}>
            {results.map((paper: Paper) => (
              <div key={paper.id} style={resultCardStyle}>
                <h3 style={resultTitleStyle}>{paper.title}</h3>
                <p style={resultMetaStyle}>
                  {paper.authors.length > 0 ? paper.authors.join(', ') : 'Unknown author'}
                  {paper.venue ? ` · ${paper.venue}` : ''}
                  {paper.publicationDate ? ` · ${paper.publicationDate}` : ''}
                  {paper.citations ? ` · ${paper.citations} citations` : ''}
                </p>
                {paper.abstract && (
                  <p style={{ ...resultMetaStyle, marginTop: '4px' }}>{paper.abstract}</p>
                )}
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  )
}

export default Search
