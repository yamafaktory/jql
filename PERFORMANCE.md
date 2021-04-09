| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 43.2 ± 1.9 | 38.5 | 60.0 | 19.50 ± 6.97 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.2 ± 0.8 | 1.5 | 16.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 43.1 ± 2.2 | 38.7 | 76.7 | 22.92 ± 5.21 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 1.9 ± 0.4 | 1.3 | 6.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 43.6 ± 2.1 | 39.5 | 55.5 | 22.24 ± 8.51 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.0 ± 0.7 | 1.3 | 16.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 196.3 ± 7.0 | 176.1 | 230.8 | 1.28 ± 0.07 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 152.8 ± 6.3 | 140.2 | 213.5 | 1.00 |

