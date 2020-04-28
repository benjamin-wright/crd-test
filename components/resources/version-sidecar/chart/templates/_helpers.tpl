{{- define "resources" -}}
resources:
  requests:
    memory: {{ .requests.memory }}
    cpu: {{ .requests.cpu }}
  limits:
    memory: {{ .limits.memory }}
    cpu: {{ .limits.cpu }}
{{- end -}}

{{- define "test.namespace" -}}
version-sidecar-test-{{ .Release.Revision }}
{{- end -}}