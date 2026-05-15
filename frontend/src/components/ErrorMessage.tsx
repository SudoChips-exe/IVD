import React, { FC } from 'react'
import '../styles/components.css'

interface ErrorMessageProps {
  message: string
}

const ErrorMessage: FC<ErrorMessageProps> = ({ message }) => {
  return (
    <div className="error-message">
      <span className="error-icon">❌</span>
      <span className="error-text">{message}</span>
    </div>
  )
}

export default ErrorMessage
