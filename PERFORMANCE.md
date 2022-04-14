| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 40.8 ± 2.6 | 37.9 | 56.8 | 16.85 ± 6.37 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.4 ± 0.9 | 1.8 | 21.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 40.3 ± 2.3 | 37.8 | 59.6 | 16.62 ± 3.79 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.4 ± 0.5 | 1.8 | 10.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 40.0 ± 2.1 | 37.5 | 59.9 | 18.35 ± 6.07 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.2 ± 0.7 | 1.6 | 14.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 201.4 ± 5.7 | 194.3 | 260.5 | 1.30 ± 0.06 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 154.7 ± 5.3 | 146.8 | 206.4 | 1.00 |

