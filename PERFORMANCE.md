| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 43.3 ± 2.9 | 39.6 | 57.4 | 15.51 ± 6.85 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.8 ± 1.2 | 1.9 | 26.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 43.1 ± 2.7 | 39.5 | 56.9 | 14.88 ± 4.84 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.9 ± 0.9 | 2.0 | 12.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 41.5 ± 1.8 | 39.3 | 55.8 | 15.85 ± 4.01 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.6 ± 0.7 | 1.8 | 14.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 211.7 ± 6.5 | 202.8 | 248.5 | 1.34 ± 0.07 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 158.0 ± 6.9 | 147.4 | 214.4 | 1.00 |

