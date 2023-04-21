| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 23.4 ± 0.2 | 23.0 | 24.4 | 13.11 ± 1.56 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 1.8 ± 0.2 | 1.5 | 3.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 23.2 ± 0.2 | 22.8 | 24.0 | 13.46 ± 1.96 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 1.7 ± 0.3 | 1.5 | 6.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 23.3 ± 0.2 | 22.9 | 27.2 | 13.73 ± 1.90 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 1.7 ± 0.2 | 1.4 | 6.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 145.9 ± 0.7 | 144.9 | 155.6 | 1.20 ± 0.03 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 121.1 ± 3.1 | 113.8 | 146.2 | 1.00 |

