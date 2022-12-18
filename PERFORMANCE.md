| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 26.3 ± 0.5 | 25.1 | 28.1 | 13.01 ± 1.85 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.0 ± 0.3 | 1.5 | 3.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 26.1 ± 0.5 | 24.8 | 27.6 | 13.42 ± 1.79 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 1.9 ± 0.3 | 1.4 | 2.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 25.9 ± 0.5 | 24.9 | 29.4 | 13.54 ± 2.63 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 1.9 ± 0.4 | 1.4 | 8.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 151.3 ± 0.8 | 150.3 | 161.7 | 1.20 ± 0.03 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 126.0 ± 3.3 | 115.3 | 141.3 | 1.00 |

