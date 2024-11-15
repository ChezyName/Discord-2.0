export function isLoggedIn() {
    return true;
}

const LoginScreen = () => {
  return (
    <div style={{width: '100%', height: '100%', backgroundColor: 'white',
        display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center'}}>
        
        <h1>Welcome Back,</h1>
        <input/>
    </div>
  )
}

export default LoginScreen