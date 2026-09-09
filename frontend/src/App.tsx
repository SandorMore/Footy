import { useEffect, useState, type FormEvent } from 'react'
import { BrowserRouter, Link, Route, Routes } from 'react-router-dom'
import './App.css'

function ThemeToggle({ darkMode, onToggle }: { darkMode: boolean; onToggle: () => void }) {
  return <button className="theme-toggle" type="button" onClick={onToggle} aria-label="Toggle colour theme"><span aria-hidden="true">{darkMode ? '☀' : '☾'}</span><span className="theme-label">{darkMode ? 'Light' : 'Dark'}</span></button>
}

function Navbar({ darkMode, onToggle }: { darkMode: boolean; onToggle: () => void }) {
  return <header className="navbar"><Link className="brand" to="/"><span className="brand-mark" aria-hidden="true">F</span><span>FOOTY<span className="brand-dot">.</span></span></Link><nav className="nav-links" aria-label="Main navigation"><Link to="/how-it-works">How it works</Link><Link to="/leaderboard">Leaderboard</Link><ThemeToggle darkMode={darkMode} onToggle={onToggle} /><Link className="button button-quiet" to="/register">Register</Link><Link className="button button-primary" to="/signup">Sign up <span aria-hidden="true">↗</span></Link></nav></header>
}

function Home() {
  return <main>
    <section className="hero-section"><div className="hero-copy"><p className="eyebrow"><span className="live-dot" /> 2026 / 27 season is open</p><h1>Build your<br /><em>ultimate XI.</em></h1><p className="hero-description">Pick your squad. Outsmart your rivals. Make every matchday count.</p><div className="hero-actions"><Link className="button button-primary button-large" to="/signup">Create your team <span aria-hidden="true">↗</span></Link><Link className="text-link" to="/how-it-works">See how it works <span aria-hidden="true">↓</span></Link></div><div className="proof-row"><div className="avatar-stack" aria-label="Players already competing"><span>JM</span><span>AK</span><span>LS</span><span>+</span></div><p><strong>48,291</strong> managers already competing</p></div></div><div className="hero-visual" aria-label="Fantasy football pitch showing an 11-player sample team"><div className="match-card"><span>GAMEWEEK 01</span><strong>YOUR SQUAD</strong><i>01:04:32</i></div><div className="pitch"><div className="pitch-line pitch-half" /><div className="pitch-line pitch-circle" /><div className="pitch-line pitch-box top" /><div className="pitch-line pitch-box bottom" /><div className="player player-1"><b>9.4</b><span>LW</span></div><div className="player player-2"><b>8.9</b><span>ST</span></div><div className="player player-3"><b>8.6</b><span>RW</span></div><div className="player player-4"><b>8.1</b><span>LM</span></div><div className="player player-5"><b>7.8</b><span>CM</span></div><div className="player player-6"><b>7.6</b><span>RM</span></div><div className="player player-7"><b>7.9</b><span>LB</span></div><div className="player player-8"><b>7.5</b><span>CB</span></div><div className="player player-9"><b>7.7</b><span>CB</span></div><div className="player player-10"><b>7.4</b><span>RB</span></div><div className="player player-11"><b>8.1</b><span>GK</span></div></div><div className="visual-caption"><span><strong>11</strong> players selected</span><span><strong>£100.0m</strong> budget</span></div></div></section>
    <section className="ticker" aria-label="League activity"><span>LIVE FROM THE LEAGUE</span><b>↑ 12.4%</b><span>AVERAGE POINTS</span><b>68.2</b><span>TEAMS CREATED</span><b>1,204,988</b></section>
  </main>
}

const managers = [
  ['01', 'Mersey Masters', '684'],
  ['02', 'The Pressing Room', '672'],
  ['03', 'FC Northside', '659'],
  ['04', 'Sunday League', '648'],
]

function Leaderboard() {
  return <main className="leaderboard-page"><section className="leaderboard-section"><div className="leaderboard-heading"><div><p className="eyebrow">Global league / gameweek 01</p><h1>The leaderboard.</h1></div><Link className="text-link" to="/signup">Join the climb <span aria-hidden="true">↗</span></Link></div><div className="leaderboard-table"><div className="leaderboard-row leaderboard-header"><span>Rank</span><span>Manager</span><span>Points</span></div>{managers.map(([rank, manager, points]) => <div className="leaderboard-row" key={rank}><span className="rank">{rank}</span><strong>{manager}</strong><span className="points">{points}<small> pts</small></span></div>)}</div></section></main>
}

function HowItWorks() {
  return <main className="how-page"><div className="how-intro"><p className="eyebrow">The Footy playbook</p><h1>Three moves.<br /><em>One season.</em></h1><p>Fantasy football without the noise. Make smart calls, trust your instincts and see how far your XI can go.</p></div><div className="how-steps"><article><span className="feature-number">01</span><span className="feature-icon">♙</span><h2>Pick your squad</h2><p>Build a team of real players within your budget. Balance proven stars with the next breakout talent.</p></article><article><span className="feature-number">02</span><span className="feature-icon">↗</span><h2>Make your moves</h2><p>Transfers, captain picks and bold decisions. Every choice matters when the whistle goes.</p></article><article><span className="feature-number">03</span><span className="feature-icon">♛</span><h2>Rule your league</h2><p>Challenge friends, climb the table and own your matchday one decision at a time.</p></article></div><Link className="button button-primary button-large" to="/signup">Build your team <span aria-hidden="true">↗</span></Link></main>
}

function AuthPage({ mode }: { mode: 'register' | 'signup' }) {
  const isSignup = mode === 'signup'
  const [message, setMessage] = useState('')

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    const formData = new FormData(event.currentTarget)
    const payload = {
      username: String(formData.get('username')),
      email: String(formData.get('email')),
      password: String(formData.get('password')),
    }

    try {
      const response = await fetch(`${import.meta.env.VITE_API_URL ?? '/api'}/register`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      })
      setMessage(response.ok ? 'Account created. Welcome to Footy.' : 'Registration failed. Check your details and try again.')
    } catch {
      setMessage('Unable to reach the Footy server.')
    }
  }

  return <main className="auth-page"><div className="auth-intro"><p className="eyebrow">{isSignup ? 'Start your season' : 'Welcome back to Footy'}</p><h1>{isSignup ? 'Create your team.' : 'Join the league.'}</h1><p>{isSignup ? 'Your squad is one decision away.' : 'Register to keep your fantasy journey moving.'}</p></div><form className="auth-form" onSubmit={handleSubmit}><label>Username<input name="username" type="text" placeholder="Your manager name" required /></label><label>Email address<input name="email" type="email" placeholder="you@example.com" required /></label><label>Password<input name="password" type="password" placeholder="At least 8 characters" minLength={8} required /></label>{isSignup && <label>Confirm password<input name="confirmPassword" type="password" placeholder="Repeat your password" minLength={8} required /></label>}<button className="button button-primary button-large" type="submit">{isSignup ? 'Create account' : 'Register now'} <span aria-hidden="true">↗</span></button>{message && <p className="form-note" role="status">{message}</p>}<p className="form-note">By continuing, you agree to the Footy league rules.</p></form><Link className="back-link" to="/">← Back to home</Link></main>
}

function App() {
  const [darkMode, setDarkMode] = useState(() => localStorage.getItem('footy-theme') === 'dark')
  useEffect(() => { document.documentElement.dataset.theme = darkMode ? 'dark' : 'light'; localStorage.setItem('footy-theme', darkMode ? 'dark' : 'light') }, [darkMode])
  return <BrowserRouter><div className="app-shell"><Navbar darkMode={darkMode} onToggle={() => setDarkMode((value) => !value)} /><Routes><Route path="/" element={<Home />} /><Route path="/how-it-works" element={<HowItWorks />} /><Route path="/leaderboard" element={<Leaderboard />} /><Route path="/register" element={<AuthPage mode="register" />} /><Route path="/signup" element={<AuthPage mode="signup" />} /></Routes></div></BrowserRouter>
}

export default App
