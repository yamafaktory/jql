| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 32.7 ± 0.6 | 31.8 | 35.6 | 17.20 ± 3.44 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 1.9 ± 0.4 | 1.5 | 9.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 32.8 ± 0.6 | 31.9 | 35.2 | 16.33 ± 2.74 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.0 ± 0.3 | 1.6 | 7.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 32.7 ± 0.6 | 31.9 | 34.9 | 16.57 ± 2.65 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.0 ± 0.3 | 1.5 | 6.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 165.6 ± 0.8 | 164.3 | 172.0 | 1.33 ± 0.02 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 124.4 ± 1.6 | 120.5 | 144.4 | 1.00 |

