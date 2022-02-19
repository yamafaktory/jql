| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 32.6 ± 0.6 | 31.5 | 37.4 | 14.54 ± 3.11 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.2 ± 0.5 | 1.7 | 5.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 33.0 ± 1.4 | 31.5 | 45.2 | 16.53 ± 3.20 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.0 ± 0.4 | 1.6 | 10.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 32.5 ± 0.8 | 31.5 | 40.9 | 16.61 ± 2.49 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.0 ± 0.3 | 1.6 | 5.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 168.8 ± 35.8 | 162.0 | 485.8 | 1.34 ± 0.28 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 126.2 ± 1.6 | 121.8 | 145.1 | 1.00 |

