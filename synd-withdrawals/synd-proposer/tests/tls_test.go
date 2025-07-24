package tests

import (
	"crypto/x509"
	"net/url"
	"testing"

	cryptotls "crypto/tls"

	"github.com/SyndicateProtocol/synd-appchains/synd-proposer/pkg/tls"
	"github.com/pkg/errors"
)

func TestIsTLSErr(t *testing.T) {
	tests := []struct {
		name     string
		err      error
		expected bool
	}{
		{
			name:     "nil error",
			err:      nil,
			expected: false,
		},
		{
			name:     "non-url error",
			err:      errors.New("some random error"),
			expected: false,
		},
		{
			name:     "url error with non-TLS error",
			err:      &url.Error{Op: "Get", URL: "https://example.com", Err: errors.New("connection refused")},
			expected: false,
		},
		{
			name: "url error with certificate invalid error",
			err: &url.Error{
				Op:  "Get",
				URL: "https://example.com",
				Err: x509.CertificateInvalidError{
					Cert:   nil,
					Reason: x509.Expired,
					Detail: "certificate has expired",
				},
			},
			expected: true,
		},
		{
			name: "url error with unknown authority error",
			err: &url.Error{
				Op:  "Get",
				URL: "https://example.com",
				Err: x509.UnknownAuthorityError{
					Cert: nil,
				},
			},
			expected: true,
		},
		{
			name: "url error with TLS record header error",
			err: &url.Error{
				Op:  "Get",
				URL: "https://example.com",
				Err: cryptotls.RecordHeaderError{Msg: "bad record header"},
			},
			expected: true,
		},
		{
			name: "wrapped url error with certificate invalid error",
			err: &url.Error{
				Op:  "Get",
				URL: "https://example.com",
				Err: &wrappedError{
					msg: "wrapped cert error",
					err: x509.CertificateInvalidError{
						Cert:   nil,
						Reason: x509.NotAuthorizedToSign,
						Detail: "certificate not authorized",
					},
				},
			},
			expected: true,
		},
		{
			name: "wrapped url error with unknown authority error",
			err: &url.Error{
				Op:  "Get",
				URL: "https://example.com",
				Err: &wrappedError{
					msg: "wrapped auth error",
					err: x509.UnknownAuthorityError{
						Cert: nil,
					},
				},
			},
			expected: true,
		},
		{
			name: "wrapped url error with TLS record header error",
			err: &url.Error{
				Op:  "Get",
				URL: "https://example.com",
				Err: &wrappedError{
					msg: "wrapped TLS error",
					err: cryptotls.RecordHeaderError{Msg: "bad TLS record"},
				},
			},
			expected: true,
		},
		{
			name: "deeply wrapped url error with certificate error",
			err: &url.Error{
				Op:  "Get",
				URL: "https://example.com",
				Err: &wrappedError{
					msg: "level 1",
					err: &wrappedError{
						msg: "level 2",
						err: x509.CertificateInvalidError{
							Cert:   nil,
							Reason: x509.CANotAuthorizedForThisName,
							Detail: "CA not authorized for this name",
						},
					},
				},
			},
			expected: true,
		},
		{
			name: "wrapped non-url error with TLS error inside",
			err: &wrappedError{
				msg: "not a url error",
				err: x509.CertificateInvalidError{
					Cert:   nil,
					Reason: x509.Expired,
					Detail: "expired",
				},
			},
			expected: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := tls.IsTLSErr(tt.err)
			if result != tt.expected {
				t.Errorf("isTLSErr() = %v, want %v", result, tt.expected)
			}
		})
	}
}

// Helper type for testing wrapped errors
type wrappedError struct {
	msg string
	err error
}

func (w *wrappedError) Error() string {
	return w.msg + ": " + w.err.Error()
}

func (w *wrappedError) Unwrap() error {
	return w.err
}

// Benchmark to ensure the function performs well
func BenchmarkIsTLSErr(b *testing.B) {
	testCases := []error{
		nil,
		errors.New("regular error"),
		&url.Error{Op: "Get", URL: "https://example.com", Err: errors.New("connection refused")},
		&url.Error{
			Op:  "Get",
			URL: "https://example.com",
			Err: x509.CertificateInvalidError{
				Cert:   nil,
				Reason: x509.Expired,
				Detail: "certificate has expired",
			},
		},
		&url.Error{
			Op:  "Get",
			URL: "https://example.com",
			Err: cryptotls.RecordHeaderError{Msg: "bad record header"},
		},
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		for _, err := range testCases {
			_ = tls.IsTLSErr(err)
		}
	}
}
