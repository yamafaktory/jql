| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 30.2 ± 1.9 | 27.9 | 39.6 | 15.60 ± 3.25 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 1.9 ± 0.4 | 1.4 | 9.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 29.9 ± 2.0 | 27.6 | 38.1 | 15.17 ± 6.13 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.0 ± 0.8 | 1.4 | 9.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 29.8 ± 1.8 | 27.7 | 35.6 | 15.93 ± 2.78 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 1.9 ± 0.3 | 1.5 | 6.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 158.2 ± 7.9 | 144.9 | 174.7 | 1.30 ± 0.09 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 121.2 ± 5.8 | 109.8 | 143.1 | 1.00 |

