import Typography from '@mui/material/Typography'

const SingleMessage = ({message, displayName, isSameAsLast, passedRef}: any) => {
  return (
    <div style={{width: "calc(100% - 24px)", height: "auto", paddingLeft: "12px", paddingRight: "12px", marginTop: (isSameAsLast ? '0px' : '8px')}}>
      {
        isSameAsLast ? "" :
        (
          <Typography color='var(--Text)' style={{fontSize: "20px", fontWeight: "bold", marginTop: "8px"}}>
              {displayName}
          </Typography>
        )
      }

        <Typography color='var(--Text)' ref={passedRef} style={{fontSize: "16px"}}>
          {message}
        </Typography>
    </div>
  )
}

export default SingleMessage